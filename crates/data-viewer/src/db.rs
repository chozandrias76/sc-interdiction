//! Database access layer for browsing `PostgreSQL` data with medallion schemas.

use crate::types::{ColumnInfo, SortDirection, TableInfo};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_types::Text;
use eyre::{Result, WrapErr};

/// Type alias for the connection pool
type DbPool = Pool<ConnectionManager<PgConnection>>;

/// The medallion schemas to query
const SCHEMAS: &[&str] = &["raw", "silver", "gold"];

/// Database connection wrapper.
pub struct DataStore {
    pool: DbPool,
}

impl DataStore {
    /// Open a database connection from environment variable.
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").wrap_err("DATABASE_URL environment variable not set")?;
        Self::open(&database_url)
    }

    /// Open a database at the given URL.
    pub fn open(database_url: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .wrap_err("Failed to create connection pool")?;
        Ok(Self { pool })
    }

    /// Get a connection from the pool.
    fn get_conn(&self) -> Result<diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        self.pool
            .get()
            .wrap_err("Failed to get database connection")
    }

    /// List all tables and views from medallion schemas with their row counts.
    pub fn list_tables(&self) -> Result<Vec<TableInfo>> {
        let mut conn = self.get_conn()?;
        let mut tables = Vec::new();

        for schema in SCHEMAS {
            // Query for tables in this schema
            let query = format!(
                r#"
                SELECT table_name, table_type
                FROM information_schema.tables
                WHERE table_schema = '{}'
                ORDER BY table_name
                "#,
                schema
            );

            let results: Vec<(String, String)> = diesel::sql_query(query)
                .load::<TableRow>(&mut *conn)?
                .into_iter()
                .map(|r| (r.table_name, r.table_type))
                .collect();

            for (name, _table_type) in results {
                let count = self.count_records_qualified(schema, &name, None)?;
                tables.push(TableInfo {
                    schema: (*schema).to_string(),
                    name,
                    row_count: count,
                });
            }
        }

        Ok(tables)
    }

    /// Get column information for a table.
    pub fn table_columns(&self, schema: &str, table: &str) -> Result<Vec<ColumnInfo>> {
        let mut conn = self.get_conn()?;

        let query = format!(
            r#"
            SELECT column_name, data_type
            FROM information_schema.columns
            WHERE table_schema = '{}' AND table_name = '{}'
            ORDER BY ordinal_position
            "#,
            Self::escape_string(schema),
            Self::escape_string(table)
        );

        let columns: Vec<ColumnInfo> = diesel::sql_query(query)
            .load::<ColumnRow>(&mut *conn)?
            .into_iter()
            .map(|r| ColumnInfo {
                name: r.column_name,
                type_name: r.data_type,
            })
            .collect();

        Ok(columns)
    }

    /// Count records in a table, optionally filtered by search.
    fn count_records_qualified(
        &self,
        schema: &str,
        table: &str,
        search: Option<&str>,
    ) -> Result<usize> {
        let mut conn = self.get_conn()?;

        let qualified = format!(
            "{}.{}",
            Self::quote_identifier(schema),
            Self::quote_identifier(table)
        );

        let sql = if let Some(query) = search {
            let columns = self.table_columns(schema, table)?;
            let where_clause = Self::build_search_where(&columns, query);
            format!(
                "SELECT COUNT(*) as cnt FROM {} WHERE {}",
                qualified, where_clause
            )
        } else {
            format!("SELECT COUNT(*) as cnt FROM {}", qualified)
        };

        let result: CountResult = diesel::sql_query(sql).get_result(&mut *conn)?;
        #[allow(clippy::cast_sign_loss)]
        Ok(result.cnt as usize)
    }

    /// Count records using `TableInfo`
    pub fn count_records(&self, table: &TableInfo, search: Option<&str>) -> Result<usize> {
        self.count_records_qualified(&table.schema, &table.name, search)
    }

    /// Fetch records with pagination and optional sorting.
    #[allow(clippy::too_many_arguments)]
    pub fn fetch_records(
        &self,
        table: &TableInfo,
        columns: &[ColumnInfo],
        limit: usize,
        offset: usize,
        search: Option<&str>,
        sort_column: Option<&str>,
        sort_direction: SortDirection,
    ) -> Result<Vec<Vec<String>>> {
        let mut conn = self.get_conn()?;

        let qualified = format!(
            "{}.{}",
            Self::quote_identifier(&table.schema),
            Self::quote_identifier(&table.name)
        );

        let where_clause = search
            .map(|q| format!("WHERE {}", Self::build_search_where(columns, q)))
            .unwrap_or_default();

        let order_clause = sort_column
            .map(|col| {
                format!(
                    "ORDER BY {} {}",
                    Self::quote_identifier(col),
                    sort_direction.sql()
                )
            })
            .unwrap_or_default();

        // Build column list as array - cast all to text for uniform handling
        let column_list: String = columns
            .iter()
            .enumerate()
            .map(|(i, c)| {
                format!(
                    "COALESCE({}::text, '<null>') as col{}",
                    Self::quote_identifier(&c.name),
                    i
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "SELECT {} FROM {} {} {} LIMIT {} OFFSET {}",
            column_list, qualified, where_clause, order_clause, limit, offset
        );

        // Use raw SQL and fetch as array of text values
        let rows = diesel::sql_query(&sql).load::<StringRow>(&mut *conn)?;

        // Convert to strings
        let records: Vec<Vec<String>> = rows.into_iter().map(|row| row.values).collect();

        Ok(records)
    }

    /// Build a WHERE clause for searching across text columns.
    fn build_search_where(columns: &[ColumnInfo], query: &str) -> String {
        let mut conditions: Vec<String> = Vec::new();
        let mut global_terms: Vec<&str> = Vec::new();

        // Parse query for special filters
        for part in query.split_whitespace() {
            if let Some(col_name) = part.strip_prefix('-').and_then(|s| s.strip_suffix(':')) {
                // -col: means column must not be null or empty
                if let Some(col) = columns
                    .iter()
                    .find(|c| c.name.eq_ignore_ascii_case(col_name))
                {
                    conditions.push(format!(
                        "({} IS NOT NULL AND {}::text != '')",
                        Self::quote_identifier(&col.name),
                        Self::quote_identifier(&col.name)
                    ));
                }
            } else if let Some(rest) = part.strip_prefix('+') {
                // +col:value means column must contain value
                if let Some((col_name, value)) = rest.split_once(':') {
                    if let Some(col) = columns
                        .iter()
                        .find(|c| c.name.eq_ignore_ascii_case(col_name))
                    {
                        let pattern = Self::glob_to_like(value);
                        conditions.push(format!(
                            "LOWER({}::text) LIKE LOWER('{}') ESCAPE '\\'",
                            Self::quote_identifier(&col.name),
                            pattern
                        ));
                    }
                }
            } else {
                global_terms.push(part);
            }
        }

        // Build global search across all columns for remaining terms
        if !global_terms.is_empty() {
            let global_query = global_terms.join(" ");
            let pattern = Self::glob_to_like(&global_query);

            let global_conditions: Vec<String> = columns
                .iter()
                .map(|c| {
                    format!(
                        "LOWER({}::text) LIKE LOWER('{}') ESCAPE '\\'",
                        Self::quote_identifier(&c.name),
                        pattern
                    )
                })
                .collect();

            if !global_conditions.is_empty() {
                conditions.push(format!("({})", global_conditions.join(" OR ")));
            }
        }

        if conditions.is_empty() {
            "1=1".to_string()
        } else {
            conditions.join(" AND ")
        }
    }

    /// Convert glob pattern to SQL LIKE pattern.
    fn glob_to_like(query: &str) -> String {
        let pattern = query
            .replace('\'', "''")
            .replace('%', "\\%")
            .replace('_', "\\_")
            .replace('*', "%")
            .replace('?', "_");

        // If no wildcards, wrap with % for substring match
        if pattern.contains('%') || pattern.contains('_') {
            pattern
        } else {
            format!("%{pattern}%")
        }
    }

    /// Quote an identifier to prevent SQL injection.
    fn quote_identifier(name: &str) -> String {
        format!("\"{}\"", name.replace('"', "\"\""))
    }

    /// Escape a string for SQL.
    fn escape_string(s: &str) -> String {
        s.replace('\'', "''")
    }
}

// Diesel query result types

#[derive(QueryableByName)]
struct TableRow {
    #[diesel(sql_type = Text)]
    table_name: String,
    #[diesel(sql_type = Text)]
    table_type: String,
}

#[derive(QueryableByName)]
struct ColumnRow {
    #[diesel(sql_type = Text)]
    column_name: String,
    #[diesel(sql_type = Text)]
    data_type: String,
}

#[derive(QueryableByName)]
struct CountResult {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    cnt: i64,
}

/// String row result for dynamic column queries.
/// Columns are aliased as col0, col1, col2, etc.
struct StringRow {
    values: Vec<String>,
}

impl diesel::deserialize::QueryableByName<diesel::pg::Pg> for StringRow {
    fn build<'a>(
        row: &impl diesel::row::NamedRow<'a, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        use diesel::row::NamedRow;

        let mut values = Vec::new();
        // Try to get columns by name (col0, col1, col2, ...) until we run out
        for i in 0..100 {
            let col_name = format!("col{}", i);
            match NamedRow::get::<Text, String>(row, &col_name) {
                Ok(value) => values.push(value),
                Err(_) => break, // No more columns
            }
        }
        Ok(StringRow { values })
    }
}
