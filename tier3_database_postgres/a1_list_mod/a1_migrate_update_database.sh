# tier3_database_postgres/a1_list_mod/a1_migrate_update_database.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_drop_function_any_param.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_list_all_constraints_foreign.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_list_all_constraints_unique.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_list_all_function_input_params.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_list_all_functions.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_list_all_table_field_definition.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_list_all_tables.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_list_all_view_fields.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a1_list_mod/a1_list_all_views.sql_vw
