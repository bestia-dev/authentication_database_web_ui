# tier3_database_postgres/a4_system_mod/a4_migrate_update_database.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/a4_system_mod/a4_random_between.sql_fn
