-- tier3_database_postgres/test_data_and functions/sql_util_for_webpage_hits.sql

select * 
-- delete
from b1_authn_signup;

select * 
-- delete
from b2_authn_login
where user_email = 'luciano.bestia@gmail.com';
