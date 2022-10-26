# Bash script for database migration/update

# sh tier3_database_postgres/migrate_update_database.sh

# Every sql file must internally check if all the criteria is met to run. 
# Here every sql script is always run in its entirety.
# The order of execution is important. 

# To create the new database and initialize or bootstrap the migration mechanism use:
# tier3_database_postgres/init/create_database_and_migration_mechanism.sql
# After bootstrap, all subsequent migration/update code will work, because the migration mechanism is already installed.

# System_wide_objects are prefixed with the namespace a_ to make it different from other namespaces like authn_ or webpage_
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_source_code.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_drop_function_any_param.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_migrate_function.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_random_between.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_list_all_view_fields.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_add_constraint_foreign_if_not_exists_one_column.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_add_constraint_unique_if_not_exists_one_column.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_list_all_constraints_unique.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/system_wide_objects/a_list_all_function_input_params.sql_vw


psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/hit_counter.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage_hits.sql_vw 
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage_hits_list.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage_hits_show.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage_hits_new.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage_hits_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage_hits_edit.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage_hits_update.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/webpage_hits/webpage_hits_delete.sql_fn

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/authn/authn_login.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/authn/authn_signup.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/authn/authn_login_show.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/authn/authn_signup_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/authn/authn_signup_delete.sql_fn



