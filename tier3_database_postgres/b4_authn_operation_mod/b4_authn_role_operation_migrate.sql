-- tier3_database_postgres/b3_authn_role_mod/b4_authn_role_operation_insert.sql_fn

-- permission for 1001-webpage_hits_admin
select * from b4_authn_role_operation_insert(1001, 1001);
select * from b4_authn_role_operation_insert(1001, 1002);
select * from b4_authn_role_operation_insert(1001, 1003);
select * from b4_authn_role_operation_insert(1001, 1004);
select * from b4_authn_role_operation_insert(1001, 1005);
select * from b4_authn_role_operation_insert(1001, 1006);
select * from b4_authn_role_operation_insert(1001, 1007);


-- TODO: role 2-readonly gets permission to all readonly operations
/*
select b4.b4_authn_operation_id
from b4_authn_operation b4
where b4.is_readonly=true;
*/
