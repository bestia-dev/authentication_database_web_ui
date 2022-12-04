-- tier3_database_postgres/a0_init_mod/a0_danger_drop_database.sql

-- DANGER !!!
-- This sql script is used only in development, when we need to drop the databse with the goal to recreate it with the migration mechanism.
-- As the name suggests, it will DROP th databse without excuses !
-- Use carefully.
-- psql -U admin -h localhost -p 5432 -d postgres -f tier3_database_postgres/a0_init_mod/a0_danger_drop_database.sql

drop database webpage_hit_counter;
