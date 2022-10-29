-- tier3_database_postgres/migrate_check.sql

-- After migrate, check if there are no strange things that are not supported for this simple database application.
-- I want to return a string and not a table

select format('function_overload: %s: %s', a.routine_name, a.count) as a3_check_function_overload from a3_check_function_overload a;
select format('multi_column_unique: %s: %s', a.constraint_name, a.count) as a3_check_multi_column_unique from a3_check_multi_column_unique a;
select format('multi_column_foreign: %s: %s', a.constraint_name, a.count) as a3_check_multi_column_foreign from a3_check_multi_column_foreign a;
select format('parameter_types: %s %s %s', a.routine_name, a.parameter_name, a.udt_name) as a3_check_parameter_types from a3_check_parameter_types a;
select format('table_field_type: %s %s %s %s', a.table_name, a.column_name, a.installed_type, a.souce_code_definition) as a3_check_table_field_type from a3_check_table_field_type a;
select format('view_field_type: %s %s %s', a.view_name, a.column_name, a.udt_name) as a3_check_view_field_type from a3_check_view_field_type a;


