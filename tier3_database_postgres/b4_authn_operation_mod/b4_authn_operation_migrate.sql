-- tier3_database_postgres/b4_authn_operation_mod/b4_authn_operation_migrate.sql

-- select * from b4_authn_operation;

select * from b4_migrate_authn_operation(1001,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_list','',true);
select * from b4_migrate_authn_operation(1002,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_new','', false);
select * from b4_migrate_authn_operation(1003,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_edit','', false);
select * from b4_migrate_authn_operation(1004,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_insert','', false);
select * from b4_migrate_authn_operation(1005,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_show','', true);
select * from b4_migrate_authn_operation(1006,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_update','', false);
select * from b4_migrate_authn_operation(1007,'/webpage_hits_admin/c1_webpage_hits_mod/c1_webpage_hits_delete','', false);
