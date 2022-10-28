# tier3_database_postgres/a2_migrate_mod/a2_migrate_update_database.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a2_migrate_mod/a2_migrate_column.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a2_migrate_mod/a2_migrate_constraint_foreign.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a2_migrate_mod/a2_migrate_constraint_unique.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a2_migrate_mod/a2_migrate_function.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a2_migrate_mod/a2_migrate_table.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a2_migrate_mod/a2_migrate_view.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a2_migrate_mod/a2_source_code.sql_tb
