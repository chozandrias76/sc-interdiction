select
      count(*) as failures,
      count(*) != 0 as should_warn,
      count(*) != 0 as should_error
    from (
      
    
    



select id
from "sc_interdiction"."raw_silver"."silver_quantum_routes"
where id is null



      
    ) dbt_internal_test