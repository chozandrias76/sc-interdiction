-- Staging model for raw shops

select
    shop_id,
    location_id,
    shop_name,
    created_at,
    updated_at
from raw.shops
