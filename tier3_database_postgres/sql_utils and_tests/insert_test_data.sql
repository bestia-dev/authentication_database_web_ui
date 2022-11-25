-- tier3_database_postgres/test_data_and functions/insert_test_data.sql

insert into c1_webpage(id, webpage)
values(555555, 'test');

insert into c1_hit_counter(webpage_id, count)
values(555555, 3);

insert into c1_webpage(id, webpage)
values(777777, 'test2');

insert into c1_hit_counter(webpage_id, count)
values(777777, 17);

select *
from b2_authn_login
