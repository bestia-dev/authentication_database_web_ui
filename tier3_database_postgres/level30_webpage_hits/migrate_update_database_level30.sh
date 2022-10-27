# tier3_database_postgres/level30_webpage_hits/migrate_update_database_level30.sh

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/hit_counter.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage_hits.sql_vw 
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage_hits_list.sql_vw
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage_hits_show.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage_hits_new.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage_hits_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage_hits_edit.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage_hits_update.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/level30_webpage_hits/webpage_hits_delete.sql_fn
