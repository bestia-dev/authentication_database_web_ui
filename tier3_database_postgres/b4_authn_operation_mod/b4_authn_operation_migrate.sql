-- tier3_database_postgres/b4_authn_operation_mod/b4_authn_operation_migrate.sql

-- select * from b4_authn_operation;

-- TODO: migrate operations

select * from b4_authn_operation_insert(1001,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_list','',true);
select * from b4_authn_operation_insert(1002,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_new','', false);
select * from b4_authn_operation_insert(1003,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_edit','', false);
select * from b4_authn_operation_insert(1004,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_insert','', false);
select * from b4_authn_operation_insert(1005,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_show','', true);
select * from b4_authn_operation_insert(1006,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_update','', false);
select * from b4_authn_operation_insert(1007,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_delete','', false);
