-- Gold location distances: calculated distances between all location pairs
-- Uses Euclidean distance formula: sqrt((x2-x1)^2 + (y2-y1)^2)
-- Distances are in millions of km (Mkm)

select
    c1.display_name as from_location,
    c1.system_name as from_system,
    c2.display_name as to_location,
    c2.system_name as to_system,
    c1.x_mkm as from_x,
    c1.y_mkm as from_y,
    c2.x_mkm as to_x,
    c2.y_mkm as to_y,
    sqrt(power(c2.x_mkm - c1.x_mkm, 2) + power(c2.y_mkm - c1.y_mkm, 2)) as distance_mkm,
    sqrt(power(c2.x_mkm - c1.x_mkm, 2) + power(c2.y_mkm - c1.y_mkm, 2)) / 149.6 as distance_au
from "sc_interdiction"."raw_silver"."location_coordinates" c1
cross join "sc_interdiction"."raw_silver"."location_coordinates" c2
where c1.display_name != c2.display_name
  and c1.system_name = c2.system_name  -- Only same-system distances (cross-system needs jump points)