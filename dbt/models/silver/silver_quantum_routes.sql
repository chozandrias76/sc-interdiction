-- Silver quantum routes: validated with foreign keys to locations

{{
    config(
        materialized='table',
        indexes=[
            {'columns': ['from_location']},
            {'columns': ['to_location']}
        ]
    )
}}

select
    r.id,
    r.from_location,
    r.to_location,
    r.distance,
    r.created_at,
    r.updated_at
from {{ ref('stg_quantum_routes') }} r
where r.from_location in (select id from {{ ref('silver_locations') }})
  and r.to_location in (select id from {{ ref('silver_locations') }})
