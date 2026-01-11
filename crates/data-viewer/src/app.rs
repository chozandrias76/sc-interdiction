//! Application state and logic.

use crate::db::DataStore;
use crate::types::{ColumnInfo, Schema, SortDirection, TableInfo, TableListItem, View};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use eyre::Result;
use std::collections::HashSet;

/// Page size for record fetching.
const PAGE_SIZE: usize = 100;

/// Application state.
pub struct App {
    /// Database connection.
    pub db: DataStore,
    /// Current view focus.
    pub view: View,
    /// Available tables.
    pub tables: Vec<TableInfo>,
    /// Expanded schema groups.
    pub expanded_schemas: HashSet<Schema>,
    /// Selected index in the visible list (headers + tables).
    pub list_index: usize,
    /// Columns of current table.
    pub columns: Vec<ColumnInfo>,
    /// Current records.
    pub records: Vec<Vec<String>>,
    /// Total record count for current table.
    pub total_records: usize,
    /// Selected record index.
    pub record_index: usize,
    /// Record offset for pagination.
    pub record_offset: usize,
    /// Selected column index (for sorting).
    pub selected_column: usize,
    /// Search query.
    pub search_query: String,
    /// Whether in search input mode.
    pub search_mode: bool,
    /// Sort column index (None = unsorted).
    pub sort_column: Option<usize>,
    /// Sort direction.
    pub sort_direction: SortDirection,
    /// Expanded column index (None = normal widths).
    pub expanded_column: Option<usize>,
    /// Status message.
    pub status: String,
    /// Error message.
    pub error: Option<String>,
}

impl App {
    /// Create a new application with the given database URL (or from env).
    pub fn new(db_url: Option<&str>) -> Result<Self> {
        let db = if let Some(url) = db_url {
            DataStore::open(url)?
        } else {
            DataStore::from_env()?
        };
        let tables = db.list_tables()?;

        let mut app = Self {
            db,
            view: View::TableList,
            tables,
            expanded_schemas: Schema::default_expanded(),
            list_index: 0,
            columns: Vec::new(),
            records: Vec::new(),
            total_records: 0,
            record_index: 0,
            record_offset: 0,
            selected_column: 0,
            search_query: String::new(),
            search_mode: false,
            sort_column: None,
            sort_direction: SortDirection::Ascending,
            expanded_column: None,
            status: "Ready".to_string(),
            error: None,
        };

        // Load first gold table if available (skip the header at index 0)
        let visible = app.visible_list_items();
        if visible.len() > 1 {
            app.list_index = 1; // First table under gold header
            if let Some(TableListItem::Table(idx)) = visible.get(1) {
                app.load_table_by_index(*idx);
            }
        }

        Ok(app)
    }

    /// Build the visible list of items (schema headers + expanded tables).
    pub fn visible_list_items(&self) -> Vec<TableListItem> {
        let mut items = Vec::new();

        for schema in Schema::display_order() {
            // Always show header
            items.push(TableListItem::SchemaHeader(schema));

            // Show tables if expanded
            if self.expanded_schemas.contains(&schema) {
                for (idx, table) in self.tables.iter().enumerate() {
                    if table.schema == schema.name() {
                        items.push(TableListItem::Table(idx));
                    }
                }
            }
        }

        items
    }

    /// Get the currently selected item in the list.
    pub fn selected_list_item(&self) -> Option<TableListItem> {
        self.visible_list_items().get(self.list_index).cloned()
    }

    /// Get the index of the currently loaded table (if any).
    pub fn current_table_index(&self) -> Option<usize> {
        match self.selected_list_item()? {
            TableListItem::Table(idx) => Some(idx),
            TableListItem::SchemaHeader(_) => None,
        }
    }

    /// Load a table's data by its index in the tables vec.
    fn load_table_by_index(&mut self, index: usize) {
        let Some(table) = self.tables.get(index) else {
            return;
        };
        let schema = table.schema.clone();
        let name = table.name.clone();

        self.record_index = 0;
        self.record_offset = 0;
        self.selected_column = 0;
        self.search_query.clear();
        self.sort_column = None;
        self.sort_direction = SortDirection::Ascending;
        self.expanded_column = None;

        match self.db.table_columns(&schema, &name) {
            Ok(cols) => self.columns = cols,
            Err(e) => {
                self.error = Some(format!("Failed to load columns: {e}"));
                return;
            }
        }

        self.refresh_records();
    }

    /// Refresh records with current pagination and search.
    fn refresh_records(&mut self) {
        let Some(table_idx) = self.current_table_index() else {
            return;
        };
        let Some(table) = self.tables.get(table_idx) else {
            return;
        };
        let search = if self.search_query.is_empty() {
            None
        } else {
            Some(self.search_query.as_str())
        };

        match self.db.count_records(table, search) {
            Ok(count) => self.total_records = count,
            Err(e) => {
                self.error = Some(format!("Failed to count records: {e}"));
                return;
            }
        }

        let sort_col = self
            .sort_column
            .and_then(|i| self.columns.get(i))
            .map(|c| c.name.as_str());

        match self.db.fetch_records(
            table,
            &self.columns,
            PAGE_SIZE,
            self.record_offset,
            search,
            sort_col,
            self.sort_direction,
        ) {
            Ok(records) => {
                self.records = records;
                self.status = format!(
                    "{} records (showing {}-{})",
                    self.total_records,
                    self.record_offset + 1,
                    (self.record_offset + self.records.len()).min(self.total_records)
                );
            }
            Err(e) => {
                self.error = Some(format!("Failed to fetch records: {e}"));
            }
        }
    }

    /// Handle a key event. Returns true if should quit.
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        // Clear error on any key
        self.error = None;

        // Handle search mode input
        if self.search_mode {
            return self.handle_search_input(key);
        }

        // Handle detail view
        if self.view == View::RecordDetail {
            return self.handle_detail_key(key);
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if self.view == View::RecordList {
                    self.view = View::TableList;
                    false
                } else {
                    true // Quit
                }
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => true,
            KeyCode::Tab => {
                self.toggle_focus();
                false
            }
            KeyCode::Char('/') | KeyCode::Char('f') => {
                self.search_mode = true;
                self.status = "Search (*/? wildcards): ".to_string();
                false
            }
            KeyCode::Char('s') => {
                self.toggle_sort();
                false
            }
            KeyCode::Char('e') => {
                self.toggle_expand();
                false
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_down();
                false
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.move_up();
                false
            }
            KeyCode::Char('h') | KeyCode::Left => {
                self.scroll_left();
                false
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.scroll_right();
                false
            }
            KeyCode::Enter => {
                self.select();
                false
            }
            KeyCode::PageDown => {
                self.page_down();
                false
            }
            KeyCode::PageUp => {
                self.page_up();
                false
            }
            KeyCode::Home => {
                self.go_home();
                false
            }
            KeyCode::End => {
                self.go_end();
                false
            }
            _ => false,
        }
    }

    fn handle_search_input(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.search_mode = false;
                self.search_query.clear();
                self.record_offset = 0;
                self.record_index = 0;
                self.refresh_records();
            }
            KeyCode::Enter => {
                self.search_mode = false;
                self.record_offset = 0;
                self.record_index = 0;
                self.refresh_records();
            }
            KeyCode::Backspace => {
                self.search_query.pop();
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
            }
            _ => {}
        }
        false
    }

    fn handle_detail_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => {
                self.view = View::RecordList;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            _ => {}
        }
        false
    }

    fn toggle_focus(&mut self) {
        self.view = match self.view {
            View::RecordList => View::TableList,
            View::TableList | View::RecordDetail => View::RecordList,
        };
    }

    fn move_down(&mut self) {
        match self.view {
            View::TableList => {
                let visible = self.visible_list_items();
                if self.list_index + 1 < visible.len() {
                    self.list_index += 1;
                }
            }
            View::RecordList => {
                if self.record_index + 1 < self.records.len() {
                    self.record_index += 1;
                } else if self.record_offset + self.records.len() < self.total_records {
                    // Load next page
                    self.record_offset += PAGE_SIZE;
                    self.record_index = 0;
                    self.refresh_records();
                }
            }
            View::RecordDetail => {}
        }
    }

    fn move_up(&mut self) {
        match self.view {
            View::TableList => {
                self.list_index = self.list_index.saturating_sub(1);
            }
            View::RecordList => {
                if self.record_index > 0 {
                    self.record_index -= 1;
                } else if self.record_offset > 0 {
                    // Load previous page
                    self.record_offset = self.record_offset.saturating_sub(PAGE_SIZE);
                    self.refresh_records();
                    self.record_index = self.records.len().saturating_sub(1);
                }
            }
            View::RecordDetail => {}
        }
    }

    fn scroll_left(&mut self) {
        if self.view == View::RecordList {
            self.selected_column = self.selected_column.saturating_sub(1);
        }
    }

    fn scroll_right(&mut self) {
        if self.view == View::RecordList && self.selected_column + 1 < self.columns.len() {
            self.selected_column += 1;
        }
    }

    fn toggle_sort(&mut self) {
        if self.view != View::RecordList || self.columns.is_empty() {
            return;
        }

        // If already sorting by this column, toggle direction or clear
        if self.sort_column == Some(self.selected_column) {
            if self.sort_direction == SortDirection::Ascending {
                self.sort_direction = SortDirection::Descending;
            } else {
                // Clear sort
                self.sort_column = None;
                self.sort_direction = SortDirection::Ascending;
            }
        } else {
            // Start sorting by selected column
            self.sort_column = Some(self.selected_column);
            self.sort_direction = SortDirection::Ascending;
        }

        self.record_offset = 0;
        self.record_index = 0;
        self.refresh_records();
    }

    fn toggle_expand(&mut self) {
        if self.view != View::RecordList || self.columns.is_empty() {
            return;
        }

        // Toggle expand on selected column
        if self.expanded_column == Some(self.selected_column) {
            self.expanded_column = None;
        } else {
            self.expanded_column = Some(self.selected_column);
        }
    }

    /// Get the name of the current sort column.
    pub fn sort_column_name(&self) -> Option<&str> {
        self.sort_column
            .and_then(|i| self.columns.get(i))
            .map(|c| c.name.as_str())
    }

    fn select(&mut self) {
        match self.view {
            View::TableList => {
                let Some(item) = self.selected_list_item() else {
                    return;
                };
                match item {
                    TableListItem::SchemaHeader(schema) => {
                        // Toggle expand/collapse
                        if self.expanded_schemas.contains(&schema) {
                            self.expanded_schemas.remove(&schema);
                        } else {
                            self.expanded_schemas.insert(schema);
                        }
                    }
                    TableListItem::Table(idx) => {
                        self.load_table_by_index(idx);
                        self.view = View::RecordList;
                    }
                }
            }
            View::RecordList => {
                if !self.records.is_empty() {
                    self.view = View::RecordDetail;
                }
            }
            View::RecordDetail => {
                self.view = View::RecordList;
            }
        }
    }

    fn page_down(&mut self) {
        if self.view == View::RecordList && self.record_offset + PAGE_SIZE < self.total_records {
            self.record_offset += PAGE_SIZE;
            self.record_index = 0;
            self.refresh_records();
        }
    }

    fn page_up(&mut self) {
        if self.view == View::RecordList && self.record_offset > 0 {
            self.record_offset = self.record_offset.saturating_sub(PAGE_SIZE);
            self.record_index = 0;
            self.refresh_records();
        }
    }

    fn go_home(&mut self) {
        if self.view == View::RecordList {
            self.record_offset = 0;
            self.record_index = 0;
            self.refresh_records();
        }
    }

    fn go_end(&mut self) {
        if self.view == View::RecordList && self.total_records > 0 {
            let last_page_start = (self.total_records / PAGE_SIZE) * PAGE_SIZE;
            self.record_offset = last_page_start;
            self.refresh_records();
            self.record_index = self.records.len().saturating_sub(1);
        }
    }

    /// Called on each tick.
    pub fn on_tick(&mut self) {
        // Nothing needed for now
    }

    /// Get the current table info.
    pub fn current_table(&self) -> Option<&TableInfo> {
        self.current_table_index()
            .and_then(|idx| self.tables.get(idx))
    }

    /// Get the current table display name.
    pub fn current_table_display(&self) -> Option<String> {
        self.current_table().map(|t| t.display_name())
    }

    /// Get the selected record.
    pub fn selected_record(&self) -> Option<&Vec<String>> {
        self.records.get(self.record_index)
    }
}
