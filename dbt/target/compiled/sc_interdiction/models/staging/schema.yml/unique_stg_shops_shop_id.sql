
    
    

select
    shop_id as unique_field,
    count(*) as n_records

from "sc_interdiction"."raw_staging"."stg_shops"
where shop_id is not null
group by shop_id
having count(*) > 1


