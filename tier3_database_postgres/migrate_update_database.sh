# Bash script for database migration/update

# sh tier3_database_postgres/migrate_update_database.sh

# Every sql file must internally check if all the criteria is met to run. 
# Here every sql script is always run in its entirety.
# The order of execution is important. 

# To create the new database and initialize or bootstrap the migration mechanism use:
# tier3_database_postgres/init/create_database_and_migration_mechanism.sql
# After bootstrap, all subsequent migration/update code will work, because the migration mechanism is already installed.
echo "Started tier3_database_postgres/migrate_update_database.sh ..."
rm tier3_database_postgres/tmp_migration_result.txt
echo "Only the actual changes are listed here. The complete output is in tier3_database_postgres/tmp_migration_result.txt." >> tier3_database_postgres/tmp_migration_result.txt

sh tier3_database_postgres/level10_system/migrate_update_database_level10.sh >> tier3_database_postgres/tmp_migration_result.txt
sh tier3_database_postgres/level20_authn/migrate_update_database_level20.sh >> tier3_database_postgres/tmp_migration_result.txt
sh tier3_database_postgres/level30_webpage_hits/migrate_update_database_level30.sh >> tier3_database_postgres/tmp_migration_result.txt


# The original output is horrible !!!
# I will process this output and show only the part that show any actual change.
grep -v "^$\|(1 row)\|Up to date.*\|a_migrate.*\|----[-]*" tier3_database_postgres/tmp_migration_result.txt

echo "Ended tier3_database_postgres/migrate_update_database.sh"
echo ""

# run checks
echo "Started tier3_database_postgres/migrate_check.sh ..."
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/migrate_check.sql
echo "Ended tier3_database_postgres/migrate_check.sh"
echo ""