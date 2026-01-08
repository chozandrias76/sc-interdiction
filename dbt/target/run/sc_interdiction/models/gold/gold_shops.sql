
  create view "sc_interdiction"."raw_gold"."gold_shops__dbt_tmp"
    
    
  as (
    -- Gold shops: denormalized view with location info

select
    s.shop_id,
    s.location_id,
    l.name as location_name,
    l.display_name as location_display_name,
    l.location_type,
    s.shop_name
from "sc_interdiction"."raw_silver"."silver_shops" s
left join "sc_interdiction"."raw_silver"."silver_locations" l on s.location_id = l.id
  );