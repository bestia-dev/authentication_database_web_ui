# tier3_database_postgres/a3_check_mod/a3_migrate_update_database.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a3_check_mod/a3_check_function_overload.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a3_check_mod/a3_check_multi_column_foreign.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a3_check_mod/a3_check_multi_column_unique.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a3_check_mod/a3_check_parameter_types.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a3_check_mod/a3_check_table_field_type.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a3_check_mod/a3_check_view_field_type.sql_vw
