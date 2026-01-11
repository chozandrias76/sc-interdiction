-- Staging model for raw quantum routes

select
    id,
    from_location,
    to_location,
    distance,
    created_at,
    updated_at
from raw.quantum_routes
