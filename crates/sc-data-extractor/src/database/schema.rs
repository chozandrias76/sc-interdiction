// @generated automatically by Diesel CLI.

pub mod raw {
    diesel::table! {
        raw.locations (id) {
            id -> Text,
            name -> Text,
            parent_id -> Nullable<Text>,
            location_type -> Text,
            nav_icon -> Nullable<Text>,
            affiliation -> Nullable<Text>,
            description -> Nullable<Text>,
            is_scannable -> Bool,
            hide_in_starmap -> Bool,
            obstruction_radius -> Nullable<Float8>,
            arrival_radius -> Nullable<Float8>,
            arrival_point_offset -> Nullable<Float8>,
            adoption_radius -> Nullable<Float8>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        raw.quantum_routes (id) {
            id -> Int4,
            from_location -> Text,
            to_location -> Text,
            distance -> Nullable<Float8>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        raw.shop_items (id) {
            id -> Int4,
            shop_id -> Text,
            item_id -> Text,
            buy_price -> Float8,
            sell_price -> Float8,
            max_inventory -> Float8,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        raw.shops (shop_id) {
            shop_id -> Text,
            location_id -> Nullable<Text>,
            shop_name -> Text,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(locations, quantum_routes, shop_items, shops,);
}

pub mod silver {
    diesel::table! {
        silver.locations (id) {
            id -> Text,
            name -> Text,
            parent_id -> Nullable<Text>,
            location_type -> Text,
            nav_icon -> Nullable<Text>,
            affiliation -> Nullable<Text>,
            description -> Nullable<Text>,
            is_scannable -> Bool,
            hide_in_starmap -> Bool,
            obstruction_radius -> Nullable<Float8>,
            arrival_radius -> Nullable<Float8>,
            arrival_point_offset -> Nullable<Float8>,
            adoption_radius -> Nullable<Float8>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            display_name -> Nullable<Text>,
        }
    }

    diesel::table! {
        silver.quantum_routes (id) {
            id -> Int4,
            from_location -> Text,
            to_location -> Text,
            distance -> Nullable<Float8>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        silver.shop_items (id) {
            id -> Int4,
            shop_id -> Text,
            item_id -> Text,
            buy_price -> Float8,
            sell_price -> Float8,
            max_inventory -> Float8,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        silver.shops (shop_id) {
            shop_id -> Text,
            location_id -> Nullable<Text>,
            shop_name -> Text,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::joinable!(shop_items -> shops (shop_id));
    diesel::joinable!(shops -> locations (location_id));

    diesel::allow_tables_to_appear_in_same_query!(locations, quantum_routes, shop_items, shops,);
}
