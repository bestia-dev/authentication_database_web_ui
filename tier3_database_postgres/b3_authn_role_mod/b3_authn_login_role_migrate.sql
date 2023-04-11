-- tier3_database_postgres/b3_authn_role_mod/b3_authn_login_role_migrate.sql

-- select * from b3_authn_login_role;

-- Login 1 is the default administrator
select b3_migrate_authn_login_role(1, 1)
