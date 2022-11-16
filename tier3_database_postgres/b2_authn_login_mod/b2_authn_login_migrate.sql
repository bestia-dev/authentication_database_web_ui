-- tier3_database_postgres/b2_authn_login_mod/b2_authn_login_migrate.sql

-- select * from b2_authn_login;

select * from b2_authn_login_insert('info@bestia.dev','hash_password');
