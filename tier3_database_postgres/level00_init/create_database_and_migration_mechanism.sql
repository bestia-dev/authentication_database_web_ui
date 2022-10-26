-- tier3_database_postgres/init/create_database_and_migration_mechanism.sql

-- This sql script contains code for the creation and initialisation of the database with a migration mechanism.
-- After that we can use the installed migration mechanism to migrate/update the database forward as we develop and deploy.

-- Run this sql script with psql. Connect to the default database 'postgres' to create the new database webpage_hit_counter. 
-- Then the script will change the current database and install other database objects.
-- psql -U admin -h localhost -p 5432 -d postgres -f tier3_database_postgres/init/create_database_and_migration_mechanism.sql

-- if the database already exists I will get an error.
-- I cannot use a DO block because of pecullar error: CREATE DATABASE cannot be executed from a function.
CREATE DATABASE webpage_hit_counter;

-- change the current database. This command only works with psql.
\c webpage_hit_counter

CREATE TABLE a_source_code
(
    object_name name,
    definition text NOT NULL,
    CONSTRAINT a_source_code_pkey PRIMARY KEY (object_name)
);

CREATE FUNCTION a_drop_function_any_param(_name name)
RETURNS text
AS
-- drop all functions with given _name regardless of function parameters
-- test it: create function test1. Then 
-- select a_drop_function_any_param('test1');   
-- drop function a_drop_function_any_param;
-- psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_drop_function_any_param.sql
$$
DECLARE
   _sql text;
   _functions_dropped int;
BEGIN
   SELECT count(*)::int
        , 'DROP FUNCTION ' || string_agg(oid::regprocedure::text, '; DROP FUNCTION ')
   FROM   pg_catalog.pg_proc
   WHERE  proname = _name
   AND    pg_function_is_visible(oid)  -- restrict to current search_path
   INTO   _functions_dropped, _sql;     -- count only returned if subsequent DROPs succeed

   IF _functions_dropped > 0 THEN       -- only if function(s) found
     EXECUTE _sql;
     return _sql;
   END IF;
   return '';
END;
$$ LANGUAGE plpgsql;

$source_code$);


CREATE FUNCTION a_migrate_function(_object_name name, _definition text)
RETURNS text 
AS
-- checks in the a_source_code if the function is already installed
-- if is equal, nothing happens
-- else drop the old and install the new function
-- finally insert/update into a_source_code  
-- psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_migrate_function.sql
$$
DECLARE
   _old_definition text;
   _x_void text;
BEGIN

   if not exists(select * from a_source_code a where a.object_name = _object_name) then
      if exists(select * from pg_proc p where p.pronamespace=2200 and p.proname=_object_name ) then
         select a_drop_function_any_param(_object_name) into _x_void;
      end if;

      EXECUTE _definition;

      insert into a_source_code (object_name, definition)
      values (_object_name, _definition);
      return format('Inserted function: %I', _object_name);
   else
      select a.definition 
      into _old_definition
      from a_source_code a
      where a.object_name = _object_name;

      if _definition <> _old_definition then
         if exists(select * from pg_proc p where p.pronamespace=2200 and p.proname=_object_name ) then
            select a_drop_function_any_param(_object_name) into _x_void;
         end if;
         
         EXECUTE _definition;

         update a_source_code
         set definition = _definition
         where object_name = _object_name;

         return format('Updated function: %I', _object_name);
      end if;

   end if;
return format('All up to date: %I', _object_name);
END;
$$ LANGUAGE plpgsql;

