-- tier3_database_postgres/b2_authn_login_mod/b2_authn_login_migrate.sql_fn

-- select * from b2_authn_login;

-- The logins under 100 are reserved to administrators, developers and special needs.
-- So the id can be used because is fixed.

-- This the first admin after installation. 
-- It can be chaned later.
select b2_migrate_authn_login(1,'info@bestia.dev','$argon2id$v=19$m=4096,t=3,p=1$000000000000000000000000$gqrluzr0Q4CAJqoMxBS8XkreuHlk9M7z1kwK1k8nEW4');

