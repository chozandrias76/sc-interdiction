
  create view "sc_interdiction"."raw_staging"."stg_shop_items__dbt_tmp"
    
    
  as (
    -- Staging model for raw shop items

select
    id,
    shop_id,
    item_id,
    buy_price,
    sell_price,
    max_inventory,
    created_at,
    updated_at
from raw.shop_items
  );