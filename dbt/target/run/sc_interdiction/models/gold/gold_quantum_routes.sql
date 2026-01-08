
  create view "sc_interdiction"."raw_gold"."gold_quantum_routes__dbt_tmp"
    
    
  as (
    -- Gold quantum routes: denormalized view with location names

select
    r.id,
    r.from_location,
    f.name as from_name,
    f.display_name as from_display_name,
    f.location_type as from_type,
    r.to_location,
    t.name as to_name,
    t.display_name as to_display_name,
    t.location_type as to_type,
    r.distance
from "sc_interdiction"."raw_silver"."silver_quantum_routes" r
left join "sc_interdiction"."raw_silver"."silver_locations" f on r.from_location = f.id
left join "sc_interdiction"."raw_silver"."silver_locations" t on r.to_location = t.id
  );