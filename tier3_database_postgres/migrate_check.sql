-- tier3_database_postgres/migrate_check.sql

-- After migrate, check if there are no strange things.

-- TODO: I need some better output format. For example none if empty

select '' as a3_check_function_overload,* from a3_check_function_overload;
select '' as a3_check_multi_column_unique ,* from a3_check_multi_column_unique;
select '' as a3_check_multi_column_foreign ,* from a3_check_multi_column_foreign;
select '' as a3_check_parameter_types ,* from a3_check_parameter_types;
select '' as a3_check_table_field_type,* from a3_check_table_field_type;
select '' as a3_check_view_field_type ,* from a3_check_view_field_type;
