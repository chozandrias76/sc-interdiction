-- Silver locations: validated with display names and corrected hierarchy
-- Uses recursive CTE to handle parent-child relationships
-- Lagrange points are re-parented to their associated planets (game data incorrectly parents to star)



with recursive location_tree as (
    -- Base case: root locations (no parent or parent doesn't exist in raw)
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
    from "sc_interdiction"."raw_staging"."stg_locations"
    where parent_id is null
       or parent_id not in (select id from "sc_interdiction"."raw_staging"."stg_locations")

    union all

    -- Recursive case: children of already-processed locations
    select
        r.id,
        r.name,
        r.parent_id,
        r.location_type,
        r.nav_icon,
        r.affiliation,
        r.description,
        r.is_scannable,
        r.hide_in_starmap,
        r.obstruction_radius,
        r.arrival_radius,
        r.arrival_point_offset,
        r.adoption_radius,
        r.created_at,
        r.updated_at
    from "sc_interdiction"."raw_staging"."stg_locations" r
    inner join location_tree lt on r.parent_id = lt.id
),

-- Get planet IDs for Lagrange point re-parenting
lagrange_planet_ids as (
    select
        lp.loc_name,
        lp.planet_loc_name,
        lp.system_name,
        stg.id as planet_id
    from "sc_interdiction"."raw_silver"."lagrange_planets" lp
    inner join "sc_interdiction"."raw_staging"."stg_locations" stg on stg.name = lp.planet_loc_name
)

select
    lt.id,
    lt.name,
    -- Re-parent Lagrange points to their associated planet, otherwise use original parent
    coalesce(
        lpi.planet_id,
        case
            when lt.parent_id in (select id from location_tree) then lt.parent_id
            else null
        end
    ) as parent_id,
    lt.location_type,
    lt.nav_icon,
    lt.affiliation,
    lt.description,
    lt.is_scannable,
    lt.hide_in_starmap,
    lt.obstruction_radius,
    lt.arrival_radius,
    lt.arrival_point_offset,
    lt.adoption_radius,
    dn.display_name,
    lt.created_at,
    lt.updated_at
from location_tree lt
left join "sc_interdiction"."raw_silver"."display_names" dn on lt.name = dn.loc_name
left join lagrange_planet_ids lpi on lt.name = lpi.loc_name