// tier0_common_code/lib.rs

// common types are used as json messages

#![deny(unused_crate_dependencies)]

use serde::{Deserialize, Serialize};

pub const APP_MAIN_ROUTE: &'static str = "webpage_hits_admin";

/// json request data for b2_authn_login_process_email
#[derive(Serialize, Deserialize)]
pub struct DataReqAuthnLoginProcessEmail {
    pub user_email: String,
}

/// json response data for b2_authn_login_process_email
#[derive(Serialize, Deserialize)]
pub struct DataRespAuthnLoginProcessEmail {
    pub salt: String,
}

/// json request data for b2_authn_login_process_hash()
#[derive(Serialize, Deserialize)]
pub struct DataReqAuthnLoginProcessHash {
    pub user_email: String,
    pub password_hash: String,
}

/// json response data for b2_authn_login_process_hash()
#[derive(Serialize, Deserialize)]
pub struct DataRespAuthnLoginProcessHash {
    pub login_success: bool,
}

/// json request data for b1_authn_signup_process_email()
#[derive(Serialize, Deserialize)]
pub struct DataReqAuthnSignupProcessEmail {
    pub user_email: String,
}

/// json response data for b1_authn_signup_process_email()
#[derive(Serialize, Deserialize)]
pub struct DataRespAuthnSignupProcessEmail {
    pub is_allowed: bool,
    pub salt: String,
}

/// json request data for b1_authn_signup_insert()
#[derive(Serialize, Deserialize)]
pub struct DataReqAuthnSignupInsert {
    pub user_email: String,
    pub password_hash: String,
}

/// json response data for b1_authn_signup_insert()
#[derive(Serialize, Deserialize)]
pub struct DataRespAuthnSignupInsert {
    pub signup_success: bool,
}
