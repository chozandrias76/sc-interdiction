select
      count(*) as failures,
      count(*) != 0 as should_warn,
      count(*) != 0 as should_error
    from (
      
    
    

select
    id as unique_field,
    count(*) as n_records

from "sc_interdiction"."raw_staging"."stg_locations"
where id is not null
group by id
having count(*) > 1



      
    ) dbt_internal_test