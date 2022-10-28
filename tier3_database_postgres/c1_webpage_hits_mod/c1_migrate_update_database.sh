# tier3_database_postgres/c1_webpage_hits_mod/c1_migrate_update_database.sh

# instalation order is important
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage.sql_tb
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_hit_counter.sql_tb

# instalation order is important
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage_hits.sql_vw 
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage_hits_list.sql_vw

psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage_hits_show.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage_hits_new.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage_hits_insert.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage_hits_edit.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage_hits_update.sql_fn
psql -U admin -h localhost -p 5432 -d webpage_hit_counter -f tier3_database_postgres/c1_webpage_hits_mod/c1_webpage_hits_delete.sql_fn
