
    
    

select
    id as unique_field,
    count(*) as n_records

from "sc_interdiction"."raw_silver"."silver_shop_items"
where id is not null
group by id
having count(*) > 1


