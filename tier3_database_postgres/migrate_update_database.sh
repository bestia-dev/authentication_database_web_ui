# Bash script for database migration/update

# sh tier3_database_postgres/migrate_update_database.sh

# Every sql file must internally check if all the criteria is met to run. 
# Here every sql script is always run in its entirety.
# The order of execution is important. 

# To create the new database and initialize or bootstrap the migration mechanism use:
# tier3_database_postgres/init/create_database_and_migration_mechanism.sql
# After bootstrap, all subsequent migration/update code will work, because the migration mechanism is already installed.
echo "start tier3_database_postgres/migrate_update_database.sh"
echo ""
sh tier3_database_postgres/level10_system/migrate_update_database_level10.sh
sh tier3_database_postgres/level20_authn/migrate_update_database_level20.sh
sh tier3_database_postgres/level30_webpage_hits/migrate_update_database_level30.sh

# TODO: the output is horrible !!!
# I need to process this output and show only the part that is NOT OK.
