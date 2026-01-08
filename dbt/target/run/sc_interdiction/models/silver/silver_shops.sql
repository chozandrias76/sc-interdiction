
  
    

  create  table "sc_interdiction"."raw_silver"."silver_shops__dbt_tmp"
  
  
    as
  
  (
    -- Silver shops: validated with foreign key integrity



select
    s.shop_id,
    case
        when s.location_id in (select id from "sc_interdiction"."raw_silver"."silver_locations")
        then s.location_id
        else null
    end as location_id,
    s.shop_name,
    s.created_at,
    s.updated_at
from "sc_interdiction"."raw_staging"."stg_shops" s
  );
  