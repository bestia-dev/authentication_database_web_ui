-- tier3_database_postgres/test_data_and functions/c1_sql_utils.sql

select * 
-- delete
from b1_authn_signup;

select * 
-- delete
from b2_authn_login
where user_email = 'luciano@bestia.dev';
