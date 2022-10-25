# Bash script for database migration/update
# Every sql file must internally check if all the criteria is met to run. 
# Here every sql script is always run in its entirety.
# The order of execution is important. 

# There are 4 steps to create a completely new database and install the migration mechanism. 
# These must be installed manually the first time - bootstrap. Only the 'create function' part of these files must be used:
# tier3_database_postgres/init/create_database.sql
# tier3_database_postgres/system_wide_objects/a_source_code.sql_tb
# tier3_database_postgres/system_wide_objects/a_drop_function_any_param.sql_fn
# tier3_database_postgres/system_wide_objects/a_migrate_function.sql_fn

# After bootstrap, all subsequent migration/update code will work, because the migration mechanism is already installed.
# Even the same functions of the migration mechanism can be now migrate to a new version using the old version of the migration mechanism.
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



