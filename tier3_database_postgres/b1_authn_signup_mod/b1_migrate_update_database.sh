# tier3_database_postgres/b1_authn_signup_mod/b1_migrate_update_database.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b1_authn_signup_mod/b1_authn_signup.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b1_authn_signup_mod/b1_authn_signup_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b1_authn_signup_mod/b1_authn_signup_delete.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b1_authn_signup_mod/b1_authn_signup_process_email.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/b1_authn_signup_mod/b1_authn_signup_email_verification.sql_fn
