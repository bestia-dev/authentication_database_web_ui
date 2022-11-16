-- tier3_database_postgres/b3_authn_role_mod/b3_authn_role_migrate.sql

-- select * from b3_authn_role;

-- TODO: migrate role 1001-webpage_hits_admin

select * from b3_authn_role_insert(1, '1-admin','all permissions');
select * from b3_authn_role_insert(2, '2-read-only','all readonly operations');
select * from b3_authn_role_insert(3, '3-user','basic permissions for every user');

select * from b3_authn_role_insert(1001, '1001-webpage_hits_admin','admin of webpage hits');
