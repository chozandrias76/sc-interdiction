-- Staging model for raw locations
-- Reads from Diesel-managed raw.locations table

select
    id,
    name,
    parent_id,
    location_type,
    nav_icon,
    affiliation,
    description,
    is_scannable,
    hide_in_starmap,
    obstruction_radius,
    arrival_radius,
    arrival_point_offset,
    adoption_radius,
    created_at,
    updated_at
from raw.locations
