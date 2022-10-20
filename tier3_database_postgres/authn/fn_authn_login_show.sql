select drop_function('authn_login_show');

create function public.authn_login_show(
_user_email varchar(100))
returns table( authn_login_id integer,
    user_email varchar(100),
    password_hash varchar(100),
    failed_attempts integer,
    blocked boolean) 
language 'plpgsql'
as $body$
declare

begin

return query 
select  t.authn_login_id,
    t.user_email,
    t.password_hash,
    t.failed_attempts,
    t.blocked
from authn_login t
where t.user_email = _user_email;

end; 
$body$;
