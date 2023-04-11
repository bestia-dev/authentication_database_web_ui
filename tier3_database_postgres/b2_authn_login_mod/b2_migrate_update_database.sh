# tier3_database_postgres/b2_authn_login_mod/b2_migrate_update_database.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b2_authn_login_mod/b2_authn_login.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b2_authn_login_mod/b2_authn_login_show.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b2_authn_login_mod/b2_authn_login_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b2_authn_login_mod/b2_migrate_authn_login.sql_fn

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b2_authn_login_mod/b2_authn_login_migrate.sql
