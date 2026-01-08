
  
    

  create  table "sc_interdiction"."raw_silver"."silver_shop_items__dbt_tmp"
  
  
    as
  
  (
    -- Silver shop items: validated with foreign key to shops



select
    si.id,
    si.shop_id,
    si.item_id,
    si.buy_price,
    si.sell_price,
    si.max_inventory,
    si.created_at,
    si.updated_at
from "sc_interdiction"."raw_staging"."stg_shop_items" si
where si.shop_id in (select shop_id from "sc_interdiction"."raw_silver"."silver_shops")
  );
  