// b2_authn_login_mod.rs
// This module is for the SCOPE "/b2_authn_login_mod/". There is a folder with the same name with html files.
// Every html file has its separate sub-module.

// For the on_click macro, I must use crate::on_click and wasm_bindgen::JsCast
use crate::on_click;
use crate::on_input;
use crate::web_sys_mod::*;
use common_code::DataRespAuthnLoginProcessEmail;
use common_code::DataRespAuthnLoginProcessHash;
use common_code::APP_MAIN_ROUTE;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const SCOPE: &'static str = "b2_authn_login_mod";

#[wasm_bindgen]
pub fn authn_login_on_start() {
    debug_write("authn_login_on_start");

    // region: add event listeners
    on_click!("btn_submit", btn_submit);
    on_input!("user_email", email_pass_on_input);
    on_input!("password", email_pass_on_input);

    // endregion: add event listeners
}

/// on key down
pub async fn email_pass_on_input() {
    let div_alert = get_html_element_by_id("div_alert");
    div_alert.set_text_content(None);
}

/// on button click
pub async fn btn_submit() {
    debug_write("btn_submit");
    let user_email = get_input_element_value_string_by_id("user_email");
    let password = get_input_element_value_string_by_id("password");
    if user_email.is_empty() || password.is_empty() {
        msg_authentication_failed("email or password empty");
        return ();
    }
    //if let Ok(salt) = send_email_get_salt(user_email.clone()).await {
    if let Ok(resp1_obj) = send_obj_get_obj::<DataRespAuthnLoginProcessEmail>(
        SCOPE,
        "b2_authn_login_process_email",
        common_code::DataReqAuthnLoginProcessEmail {
            user_email: user_email.clone(),
        },
    )
    .await
    {
        let hash = calculate_hash(user_email.clone(), password, resp1_obj.salt);

        if let Ok(resp2_obj) = send_obj_get_obj::<DataRespAuthnLoginProcessHash>(
            SCOPE,
            "b2_authn_login_process_hash",
            common_code::DataReqAuthnLoginProcessHash {
                user_email: user_email.clone(),
                hash,
            },
        )
        .await
        {
            if resp2_obj.login_success == false {
                msg_authentication_failed("Authentication failed !");
                return ();
            } else {
                window()
                    .open_with_url_and_target(
                        &format!("/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list"),
                        "_self",
                    )
                    .unwrap();
            }
        }
    }
}

fn calculate_hash(user_email: String, password: String, salt: String) -> String {
    let input_password = format!("{user_email}_{password}");
    let argon2 = argon2::Argon2::default();
    let hash = argon2::PasswordHasher::hash_password(&argon2, &input_password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    hash
}
