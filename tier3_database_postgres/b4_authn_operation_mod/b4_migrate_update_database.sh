# tier3_database_postgres/b2_authn_login_mod/b2_migrate_update_database.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b4_authn_operation_mod/b4_authn_operation.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b4_authn_operation_mod/b4_authn_operation_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b4_authn_operation_mod/b4_migrate_authn_operation.sql_fn

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b4_authn_operation_mod/b4_authn_operation_migrate.sql

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b4_authn_operation_mod/b4_authn_role_operation.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b4_authn_operation_mod/b4_authn_role_operation_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b4_authn_operation_mod/b4_migrate_authn_role_operation.sql_fn

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b4_authn_operation_mod/b4_authn_role_operation_migrate.sql
