
  create view "sc_interdiction"."raw_staging"."stg_shops__dbt_tmp"
    
    
  as (
    -- Staging model for raw shops

select
    shop_id,
    location_id,
    shop_name,
    created_at,
    updated_at
from raw.shops
  );