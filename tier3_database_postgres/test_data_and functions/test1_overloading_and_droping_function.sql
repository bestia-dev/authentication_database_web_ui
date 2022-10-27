-- tier3_database_postgres/test_data_and functions/test1_overloading_and_droping_function.sql

create or replace function  test1()
returns table(routine_name name) 
language plpgsql as
$$
-- just a test function so I can test how to drop it
-- with a_drop_function_any_param
-- select * from test1()
declare

begin
  return query 
select p.routine_name::name from information_schema.routines p;
end
$$;
----------------------------------------------------------------------------
create or replace function  test1(_name name)
returns table(routine_name name) 
language plpgsql as
$$
-- just a test function so I can test how to drop it
-- with a_drop_function_any_param
-- select * from test1('x')
declare

begin
  return query 
select p.routine_name::name from information_schema.routines p where p.routine_name=_name;
end
$$;
----------------------------------------------------------------------------

select * from a_check_function_overload ;
----------------------------------------------------------------------------

select a_drop_function_any_param('test1');


