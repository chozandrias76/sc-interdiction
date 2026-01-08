// Schema metadata

pub const STARMAP_FIELD_COUNT: usize = 43;
pub const SHOP_FIELD_COUNT: usize = 3;
pub const GENERATED_SQL: &str = include_str!(concat!(env!("OUT_DIR"), "/generated_schema.sql"));
