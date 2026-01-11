-- Gold shops: denormalized view with location info

select
    s.shop_id,
    s.location_id,
    l.name as location_name,
    l.display_name as location_display_name,
    l.location_type,
    s.shop_name
from {{ ref('silver_shops') }} s
left join {{ ref('silver_locations') }} l on s.location_id = l.id
