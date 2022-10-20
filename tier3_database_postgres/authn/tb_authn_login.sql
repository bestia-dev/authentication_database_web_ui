CREATE TABLE IF NOT EXISTS public.authn_login
(
    authn_login_id SERIAL,
    user_email varchar(100) NOT NULL,
    password_hash varchar(100) NOT NULL,
    failed_attempts integer DEFAULT 0 NOT NULL,
    blocked boolean DEFAULT FALSE NOT NULL,
    CONSTRAINT authn_login_pkey PRIMARY KEY (authn_login_id) 
)
