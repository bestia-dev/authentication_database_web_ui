# tier3_database_postgres/level10_system/migrate_update_database_level10.sh

# level10_system are prefixed with the namespace a_ to make it different from other namespaces like authn_ or webpage_
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_source_code.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_drop_function_any_param.sql_fn

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_migrate_function.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_migrate_column.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_migrate_table.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_migrate_view.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_migrate_constraint_foreign.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_migrate_constraint_unique.sql_fn

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_list_all_tables.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_list_all_table_field_definition.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_list_all_constraints_unique.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_list_all_constraints_foreign.sql

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_list_all_views.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_list_all_view_fields.sql_vw

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_list_all_functions.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_list_all_function_input_params.sql_vw

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_check_function_overload.sql
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_check_multi_column_unique.sql
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_check_multi_column_foreign.sql

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level10_system/a_random_between.sql_fn
