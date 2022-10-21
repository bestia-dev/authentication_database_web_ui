// common_code/lib.rs

// common types are used as json messages

use serde::Deserialize;
use serde::Serialize;

/// json request data for authn_login_process_email
#[derive(Serialize, Deserialize)]
pub struct DataReqAuthnLoginProcessEmail {
    pub user_email: String,
}

/// json response data for authn_login_process_email
#[derive(Serialize, Deserialize)]
// TODO: the response must be enum like Result with ok and err variant 2022-10-21
pub struct DataRespAuthnLoginProcessEmail {
    pub salt: String,
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
