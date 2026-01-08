-- Gold shop items: denormalized view with shop and location info

select
    si.id,
    si.shop_id,
    s.shop_name,
    s.location_id,
    l.name as location_name,
    l.display_name as location_display_name,
    si.item_id,
    si.buy_price,
    si.sell_price,
    si.max_inventory,
    case
        when si.buy_price > 0 and si.sell_price > 0
        then round(((si.sell_price - si.buy_price) / si.buy_price * 100)::numeric, 2)
        else 0
    end as profit_margin_pct
from {{ ref('silver_shop_items') }} si
join {{ ref('silver_shops') }} s on si.shop_id = s.shop_id
left join {{ ref('silver_locations') }} l on s.location_id = l.id
