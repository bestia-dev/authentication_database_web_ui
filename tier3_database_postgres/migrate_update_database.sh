# Bash script for database migration/update

# sh tier3_database_postgres/migrate_update_database.sh

# Every sql file must internally check if all the criteria is met to run. 
# Here every sql script is always run in its entirety.
# The order of execution is important. 

# To create the new database and initialize or bootstrap the migration mechanism use:
# tier3_database_postgres/init/create_database_and_migration_mechanism.sql
# After bootstrap, all subsequent migration/update code will work, because the migration mechanism is already installed.

sh tier3_database_postgres/level01_system/migrate_update_database_level01.sh
sh tier3_database_postgres/level02_authn/migrate_update_database_level02.sh
sh tier3_database_postgres/level03_webpage_hits/migrate_update_database_level03.sh
