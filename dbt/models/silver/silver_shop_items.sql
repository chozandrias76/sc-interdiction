-- Silver shop items: validated with foreign key to shops

{{
    config(
        materialized='table',
        indexes=[
            {'columns': ['shop_id']},
            {'columns': ['item_id']}
        ]
    )
}}

select
    si.id,
    si.shop_id,
    si.item_id,
    si.buy_price,
    si.sell_price,
    si.max_inventory,
    si.created_at,
    si.updated_at
from {{ ref('stg_shop_items') }} si
where si.shop_id in (select shop_id from {{ ref('silver_shops') }})
