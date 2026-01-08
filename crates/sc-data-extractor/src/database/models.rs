//! Diesel models for the raw schema
//! Silver and gold layers are managed by dbt

use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::database::schema::raw::{locations, quantum_routes, shop_items, shops};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub location_type: String,
    pub nav_icon: Option<String>,
    pub affiliation: Option<String>,
    pub description: Option<String>,
    pub is_scannable: bool,
    pub hide_in_starmap: bool,
    pub obstruction_radius: Option<f64>,
    pub arrival_radius: Option<f64>,
    pub arrival_point_offset: Option<f64>,
    pub adoption_radius: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = locations)]
pub struct NewLocation<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub parent_id: Option<&'a str>,
    pub location_type: &'a str,
    pub nav_icon: Option<&'a str>,
    pub affiliation: Option<&'a str>,
    pub description: Option<&'a str>,
    pub is_scannable: bool,
    pub hide_in_starmap: bool,
    pub obstruction_radius: Option<f64>,
    pub arrival_radius: Option<f64>,
    pub arrival_point_offset: Option<f64>,
    pub adoption_radius: Option<f64>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = shops)]
#[diesel(primary_key(shop_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Shop {
    pub shop_id: String,
    pub location_id: Option<String>,
    pub shop_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = shops)]
pub struct NewShop<'a> {
    pub shop_id: &'a str,
    pub location_id: Option<&'a str>,
    pub shop_name: &'a str,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = shop_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShopItem {
    pub id: i32,
    pub shop_id: String,
    pub item_id: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub max_inventory: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = shop_items)]
pub struct NewShopItem<'a> {
    pub shop_id: &'a str,
    pub item_id: &'a str,
    pub buy_price: f64,
    pub sell_price: f64,
    pub max_inventory: f64,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = quantum_routes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QuantumRoute {
    pub id: i32,
    pub from_location: String,
    pub to_location: String,
    pub distance: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = quantum_routes)]
pub struct NewQuantumRoute<'a> {
    pub from_location: &'a str,
    pub to_location: &'a str,
    pub distance: Option<f64>,
}
