# tier3_database_postgres/level01_system/migrate_update_database_level01.sh

# level01_system are prefixed with the namespace a_ to make it different from other namespaces like authn_ or webpage_
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_source_code.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_drop_function_any_param.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_migrate_function.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_random_between.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_list_all_view_fields.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_add_constraint_foreign_if_not_exists_one_column.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_add_constraint_unique_if_not_exists_one_column.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_list_all_constraints_unique.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level01_system/a_list_all_function_input_params.sql_vw
