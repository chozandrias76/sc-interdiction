-- Gold locations: denormalized view with parent names

select
    l.id,
    l.name,
    l.display_name,
    l.parent_id,
    p.name as parent_name,
    p.display_name as parent_display_name,
    l.location_type,
    l.nav_icon,
    l.affiliation,
    l.description,
    l.is_scannable,
    l.hide_in_starmap,
    l.obstruction_radius,
    l.arrival_radius,
    l.arrival_point_offset,
    l.adoption_radius
from {{ ref('silver_locations') }} l
left join {{ ref('silver_locations') }} p on l.parent_id = p.id
