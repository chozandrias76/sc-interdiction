
  create view "sc_interdiction"."raw_staging"."stg_quantum_routes__dbt_tmp"
    
    
  as (
    -- Staging model for raw quantum routes

select
    id,
    from_location,
    to_location,
    distance,
    created_at,
    updated_at
from raw.quantum_routes
  );