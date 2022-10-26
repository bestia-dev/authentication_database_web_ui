-- tier3_database_postgres/test_data_and functions/insert_test_data.sql

insert into webpage(id, webpage)
values(555555, 'test');

insert into hit_counter(webpage_id, count)
values(555555, 3);

insert into webpage(id, webpage)
values(777777, 'test2');

insert into hit_counter(webpage_id, count)
values(777777, 17);

insert into authn_login(user_email, password_hash)
values('info@bestia.dev', '$argon2id$v=19$m=4096,t=3,p=1$000000000000000000000000$gqrluzr0Q4CAJqoMxBS8XkreuHlk9M7z1kwK1k8nEW4')

