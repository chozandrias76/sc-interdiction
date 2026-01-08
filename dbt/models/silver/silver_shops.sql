-- Silver shops: validated with foreign key integrity

{{
    config(
        materialized='table',
        indexes=[
            {'columns': ['location_id']}
        ]
    )
}}

select
    s.shop_id,
    case
        when s.location_id in (select id from {{ ref('silver_locations') }})
        then s.location_id
        else null
    end as location_id,
    s.shop_name,
    s.created_at,
    s.updated_at
from {{ ref('stg_shops') }} s
