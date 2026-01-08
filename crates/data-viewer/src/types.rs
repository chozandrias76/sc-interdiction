//! Type definitions for the data viewer.

use std::collections::HashSet;

/// Current view/focus state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    /// Focus on table list sidebar.
    #[default]
    TableList,
    /// Focus on records list.
    RecordList,
    /// Viewing a single record's details.
    RecordDetail,
}

/// Database schema tier (medallion architecture).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Schema {
    Gold,
    Silver,
    Raw,
}

impl Schema {
    /// Get the schema name as used in the database.
    pub fn name(&self) -> &'static str {
        match self {
            Schema::Gold => "gold",
            Schema::Silver => "silver",
            Schema::Raw => "raw",
        }
    }

    /// Get schemas in display order (gold, silver, raw).
    pub fn display_order() -> [Schema; 3] {
        [Schema::Gold, Schema::Silver, Schema::Raw]
    }

    /// Parse schema from string.
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "gold" => Some(Schema::Gold),
            "silver" => Some(Schema::Silver),
            "raw" => Some(Schema::Raw),
            _ => None,
        }
    }

    /// Get default expanded state (gold is expanded by default).
    pub fn default_expanded() -> HashSet<Schema> {
        let mut set = HashSet::new();
        set.insert(Schema::Gold);
        set
    }
}

/// An item in the table list (either a schema header or a table).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableListItem {
    /// A collapsible schema header.
    SchemaHeader(Schema),
    /// A table entry (index into the tables vec).
    Table(usize),
}

/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl SortDirection {
    /// Toggle between ascending and descending.
    #[allow(dead_code)]
    pub fn toggle(&self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }

    /// Get SQL ORDER BY keyword.
    pub fn sql(&self) -> &'static str {
        match self {
            Self::Ascending => "ASC",
            Self::Descending => "DESC",
        }
    }

    /// Get display symbol.
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Ascending => "^",
            Self::Descending => "v",
        }
    }
}

/// Information about a database table.
#[derive(Debug, Clone)]
pub struct TableInfo {
    /// Schema name (e.g., "raw", "silver", "gold")
    pub schema: String,
    /// Table or view name
    pub name: String,
    /// Number of rows
    pub row_count: usize,
}

impl TableInfo {
    /// Get the fully qualified name (schema.table)
    #[allow(dead_code)]
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.schema, self.name)
    }

    /// Get display name for UI
    pub fn display_name(&self) -> String {
        format!("[{}] {}", self.schema, self.name)
    }
}

/// Information about a table column.
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    #[allow(dead_code)]
    pub type_name: String,
}
