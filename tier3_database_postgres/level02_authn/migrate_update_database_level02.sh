# tier3_database_postgres/level02_authn/migrate_update_database_level02.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level02_authn/authn_login.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level02_authn/authn_signup.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level02_authn/authn_login_show.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level02_authn/authn_signup_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level02_authn/authn_signup_delete.sql_fn
