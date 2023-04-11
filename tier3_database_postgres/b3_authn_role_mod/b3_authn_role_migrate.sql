-- tier3_database_postgres/b3_authn_role_mod/b3_authn_role_migrate.sql

-- select * from b3_authn_role;

-- Roles under 100 are for administrators, developers and other special needs.

select * from b3_migrate_authn_role(1, '1-admin','all permissions');
select * from b3_migrate_authn_role(2, '2-read-only','all readonly operations');
select * from b3_migrate_authn_role(3, '3-user','basic permissions for every user');

--Roles over 100 are for other database content.
select * from b3_migrate_authn_role(1001, '1001-webpage_hits_admin','admin of webpage hits');
