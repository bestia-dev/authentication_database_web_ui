-- tier3_database_postgres/b2_authn_login_mod/b2_authn_login_migrate.sql_fn

-- select * from b2_authn_login;

-- The logins under 100 are reserved to administrators, developers and special needs.
-- So the id can be used because is fixed.

-- This the first admin after installation. 
-- It can be chaned later.
select * from b2_migrate_authn_login(1,'info@bestia.dev','hash_password');
