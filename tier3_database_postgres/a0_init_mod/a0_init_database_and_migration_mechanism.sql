-- tier3_database_postgres/a0_init_mod/a0_init_database_and_migration_mechanism.sql

-- This sql script contains code for the creation and initialisation of the database with a minimal migration mechanism.
-- After that we can use the installed migration mechanism to migrate/update the database forward as we develop and deploy.

-- Only ONCE run when creating a new database: install prerequisits for minimal migration mechanism. 
-- Connect to the default database 'postgres' to create the new database webpage_hit_counter. 
-- Then the script will change the current database and install other database objects.

-- psql -U admin -h localhost -p 5432 -d postgres -f tier3_database_postgres/a0_init_mod/a0_init_database_and_migration_mechanism.sql

create database webpage_hit_counter;

-- change the current database. This command only works with psql.
\c webpage_hit_counter;

create table a2_source_code
(
    object_name name,
    definition text not null,
    constraint a_source_code_pkey primary key (object_name)
);

create view a1_list_all_functions
as
-- only public functions
-- select * from a1_list_all_functions ;

select t.routine_name::name, 
t.specific_name::name, 
t.type_udt_name::name
from information_schema.routines t
where t.routine_schema='public' and t.routine_type='FUNCTION'
order by t.routine_name;

create view a1_list_all_views
as
-- only public views
-- select * from a1_list_all_views ;

select t.table_name::name as view_name
from information_schema.views t
where t.table_schema='public'
order by t.table_name;

create function a1_drop_function_any_param(_name name)
returns text
as
-- drop all functions with given _name regardless of function parameters
-- test it: create function test1. Then 
-- select a1_drop_function_any_param('test1');   
-- drop function a1_drop_function_any_param;
-- psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_drop_function_any_param.sql_fn
$$
declare
   _sql text;
   _functions_dropped int;
begin
   select count(*)::int
        , 'DROP function ' || string_agg(oid::regprocedure::text, '; DROP function ')
   from   pg_catalog.pg_proc
   where  proname = _name
   and    pg_function_is_visible(oid)  -- restrict to current search_path
   into   _functions_dropped, _sql;     -- count only returned if subsequent DROPs succeed

   if _functions_dropped > 0 then       -- only if function(s) found
     execute _sql;
     return _sql;
   end if;
   return '';
end;
$$ language plpgsql;


create or replace function a2_migrate_function(_object_name name, _definition text)
returns text 
as
-- checks in the a2_source_code if the function is already installed
-- if is equal, nothing happens
-- else drop the old and install the new function
-- finally insert/update into a2_source_code  
-- psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a2_migrate_mod/a2_migrate_function.sql_fn
$$
declare
   _old_definition text;
   _x_void text;
begin

   if not exists(select * from a2_source_code a where a.object_name = _object_name) then
      if exists(select * from a1_list_all_functions p where p.routine_name=_object_name) then
         select a1_drop_function_any_param(_object_name) into _x_void;
      end if;

      execute _definition;

      insert into a2_source_code (object_name, definition)
      values (_object_name, _definition);
      return format('Inserted function: %I', _object_name);
   else
      select a.definition 
      into _old_definition
      from a2_source_code a
      where a.object_name = _object_name;

      if _definition <> _old_definition then
         if exists(select * from a1_list_all_functions p where p.routine_name=_object_name) then
            select a1_drop_function_any_param(_object_name) into _x_void;
         end if;
         
         execute _definition;

         update a2_source_code
         set definition = _definition
         where object_name = _object_name;

         return format('Updated function: %I', _object_name);
      end if;

   end if;
return format('Up to date Function: %I', _object_name);
end;
$$ language plpgsql;


create function a2_migrate_view(_object_name name, _definition text)
returns text 
AS
-- checks in the a2_source_code if the view is already installed
-- if is equal, nothing happens
-- else drop the old and install the new view
-- finally insert/update into a2_source_code  
$$
declare
   _old_definition text;
   _x_void text;
begin

   if not exists(select * from a2_source_code a where a.object_name = _object_name) then
      if exists(select * from a1_list_all_views v where v.view_name=_object_name) then
         execute format('DROP VIEW %I CASCADE', _object_name);
      end if;

      execute _definition;

      insert into a2_source_code (object_name, definition)
      values (_object_name, _definition);
      return format('Inserted view: %I', _object_name);
   else
      select a.definition 
      into _old_definition
      from a2_source_code a
      where a.object_name = _object_name;

      if _definition <> _old_definition then
         if exists(select * from a1_list_all_views v where v.view_name=_object_name) then
            execute format('DROP VIEW %I CASCADE', _object_name);
         end if;
         
         execute _definition;

         update a2_source_code
         set definition = _definition
         where object_name = _object_name;

         return format('Updated view: %I', _object_name);
      end if;

   end if;
   return format('Up to date View: %I', _object_name);
end;
$$ language plpgsql;
