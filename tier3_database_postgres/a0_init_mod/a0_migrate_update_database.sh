# tier3_database_postgres/a0_init_mod/a0_migrate_update_database.sh

# Only ONCE run when creating a new database: install prerequisits for complete migration mechanism.

psql -U admin -h localhost -p 5432 -d postgres -f tier3_database_postgres/a0_init_mod/a0_init_database_and_migration_mechanism.sql
