// common_code/lib.rs

// common types are used as json messages

use serde::Deserialize;
use serde::Serialize;

/// json request data for authn_login_process_email
#[derive(Serialize, Deserialize)]
pub struct DataReqAuthnLoginProcessEmail {
    pub user_email: String,
}

/* /// json response data for authn_login_process_email
#[derive(Serialize, Deserialize)]
pub struct DataRespAuthnLoginProcessEmail {
    pub salt: String,
} */

/// json Result response data for authn_login_process_email
#[derive(Serialize, Deserialize)]
pub enum ResultAuthnLoginProcessEmail {
    Data { salt: String },
    Error(String),
}

/// json request data for authn_login_process_hash
#[derive(Serialize, Deserialize)]
pub struct DataReqAuthnLoginProcessHash {
    pub user_email: String,
    pub hash: String,
}

/// json response data for authn_login_process_hash
#[derive(Serialize, Deserialize)]
pub struct DataRespAuthnLoginProcessHash {
    pub login_success: bool,
}
