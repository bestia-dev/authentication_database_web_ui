-- tier3_database_postgres/test_data_and functions/test1_overloading_and_droping_function.sql

-- I think function overloading is dificult to maintain. 
-- For this simple database application I limit function to NOT use overloading. For simplicity.

create or replace function  test1()
returns table(routine_name name) 
as
$$
-- just a test function so I can test how to drop it
-- with a1_drop_function_any_param
-- select * from test1()
declare

begin
  return query 
select p.routine_name from a1_list_all_functions p;
end
$$ language plpgsql;
----------------------------------------------------------------------------
create or replace function  test1(_name name)
returns table(routine_name name) 
as
$$
-- just a test function so I can test how to drop it
-- with a1_drop_function_any_param
-- select * from test1('x')
declare

begin
  return query 
select p.routine_name from a1_list_all_functions p where p.routine_name=_name;
end
$$ language plpgsql;
----------------------------------------------------------------------------

select * from a3_check_function_overload ;
----------------------------------------------------------------------------

select a1_drop_function_any_param('test1');


