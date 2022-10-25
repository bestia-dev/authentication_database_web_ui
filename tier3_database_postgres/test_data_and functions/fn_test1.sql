select a_drop_function_any_param('test1');

create function  test1(_name text)
returns table(oid integer) 
LANGUAGE plpgsql AS
$$
-- just a test function so I can test how to drop it
-- with a_drop_function_any_param
DECLARE

BEGIN
  return query 
select oid FROM pg_proc p;
END
$$;