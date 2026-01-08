
  
    

  create  table "sc_interdiction"."raw_silver"."silver_quantum_routes__dbt_tmp"
  
  
    as
  
  (
    -- Silver quantum routes: validated with foreign keys to locations



select
    r.id,
    r.from_location,
    r.to_location,
    r.distance,
    r.created_at,
    r.updated_at
from "sc_interdiction"."raw_staging"."stg_quantum_routes" r
where r.from_location in (select id from "sc_interdiction"."raw_silver"."silver_locations")
  and r.to_location in (select id from "sc_interdiction"."raw_silver"."silver_locations")
  );
  