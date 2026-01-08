select
      count(*) as failures,
      count(*) != 0 as should_warn,
      count(*) != 0 as should_error
    from (
      
    
    



select id
from "sc_interdiction"."raw_staging"."stg_shop_items"
where id is null



      
    ) dbt_internal_test