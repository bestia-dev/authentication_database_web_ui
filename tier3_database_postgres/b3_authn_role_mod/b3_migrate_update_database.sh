# tier3_database_postgres/b2_authn_login_mod/b2_migrate_update_database.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b3_authn_role_mod/b3_authn_role.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b3_authn_role_mod/b3_authn_role_insert.sql_fn

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b3_authn_role_mod/b3_authn_role_migrate.sql

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b3_authn_role_mod/b3_authn_login_role.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b3_authn_role_mod/b3_authn_login_role_insert.sql_fn
