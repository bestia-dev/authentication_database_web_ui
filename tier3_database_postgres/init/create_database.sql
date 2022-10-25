-- create_database.sql

-- connect to the default database 'postgres' to create the database webpage_hit_counter
-- psql -U admin -h localhost -p 5432 -d postgres -f tier3_database_postgres/init/create_database.sql
-- if the database already exists I will get an error.
-- I cannot use a DO block because of pecullar error: CREATE DATABASE cannot be executed from a function.

CREATE DATABASE webpage_hit_counter;

