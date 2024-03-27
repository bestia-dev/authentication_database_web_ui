# Bash script for database migration/update

# Only to create a new database and initialize or bootstrap the migration mechanism use:
# sh tier3_database_postgres/a0_init_mod/a0_migrate_update_database.sh
# After bootstrap, all subsequent migration/update code will work, because the migration mechanism is already installed.

# Every sql file must internally check if all the criteria is met to run. 
# Here every sql script is always run in its entirety.
# The order of execution is important. 

# sh tier3_database_postgres/migrate_update_database.sh

printf "Started tier3_database_postgres/migrate_update_database.sh ...\n"
rm tier3_database_postgres/tmp_migration_result.txt
printf "Only the actual changes are listed here. The complete output is in tier3_database_postgres/tmp_migration_result.txt.\n"

sh tier3_database_postgres/a1_list_mod/a1_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt
sh tier3_database_postgres/a2_migrate_mod/a2_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt
sh tier3_database_postgres/a3_check_mod/a3_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt
sh tier3_database_postgres/a4_system_mod/a4_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt

sh tier3_database_postgres/b1_authn_signup_mod/b1_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt
sh tier3_database_postgres/b2_authn_login_mod/b2_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt
sh tier3_database_postgres/b3_authn_role_mod/b3_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt
sh tier3_database_postgres/b4_authn_operation_mod/b4_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt

sh tier3_database_postgres/c1_webpage_hits_mod/c1_migrate_update_database.sh >> tier3_database_postgres/tmp_migration_result.txt

# The original output is horrible !!!
# I will process this output and show only the part that show any actual change.
grep -v "^$\|(1 row)\|Up to date.*\|[ab][0-9]_migrate.*\|----[-]*" tier3_database_postgres/tmp_migration_result.txt

printf "Ended tier3_database_postgres/migrate_update_database.sh\n"
printf "\n"

# run checks
printf "Started tier3_database_postgres/migrate_check.sh ...\n"
printf "Only the actual found problems are listed here. The complete output is in tier3_database_postgres/tmp_migration_check.txt.\n"
rm tier3_database_postgres/tmp_migration_check.txt
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/migrate_check.sql >> tier3_database_postgres/tmp_migration_check.txt
# I will process this output and show only the part that show any actual change.
grep -v "^$\|(0 rows)\|a3_check.*\|----[-]*" tier3_database_postgres/tmp_migration_check.txt
printf "Ended tier3_database_postgres/migrate_check.sh\n"
printf "\n"