// tier0_common_code/lib.rs

// common types are used as json messages

#![deny(unused_crate_dependencies)]

use serde::Deserialize;
use serde::Serialize;

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

/// json request data for b2_authn_login_process_hash
#[derive(Serialize, Deserialize)]
pub struct DataReqAuthnLoginProcessHash {
    pub user_email: String,
    pub hash: String,
}

/// json response data for b2_authn_login_process_hash
#[derive(Serialize, Deserialize)]
pub struct DataRespAuthnLoginProcessHash {
    pub login_success: bool,
}
