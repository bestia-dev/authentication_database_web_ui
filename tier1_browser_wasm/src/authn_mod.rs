// authn_mod.rs
// This module is for the SCOPE "/authn/". There is a folder with the same name with html files.
// Every html file has its separate sub-module.

// wasm sub-module for authn_login.html
pub mod authn_login {
    // For the on_click macro, I must use crate::on_click and wasm_bindgen::JsCast
    use crate::on_click;
    use crate::on_input;
    use crate::web_sys_mod::*;
    use crate::APP_MAIN_ROUTE;
    use anyhow::anyhow;
    use common_code::DataRespAuthnLoginProcessEmail;
    use common_code::DataRespAuthnLoginProcessHash;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    const SCOPE: &'static str = "authn";

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
        if let Ok(salt) = send_email_get_salt(user_email.clone()).await {
            let hash = calculate_hash(user_email.clone(), password, salt);

            if let Ok(login_success) = send_email_hash_get_authn_success(user_email, hash).await {
                if login_success == false {
                    msg_authentication_failed("Authentication failed !");
                    return ();
                } else {
                    window()
                        .open_with_url_and_target(
                            &format!("/{APP_MAIN_ROUTE}/webpage_hits/webpage_hits_list"),
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
        let hash =
            argon2::PasswordHasher::hash_password(&argon2, &input_password.as_bytes(), &salt)
                .unwrap()
                .to_string();
        hash
    }

    // send user_email to server authn_login_process_email and get the salt
    async fn send_email_get_salt(user_email: String) -> anyhow::Result<String> {
        let json_body = serde_json::to_string(&common_code::DataReqAuthnLoginProcessEmail {
            user_email: user_email.clone(),
        })?;

        let resp_string = fetch_json_response(
            format!("/{APP_MAIN_ROUTE}/{SCOPE}/authn_login_process_email"),
            json_body,
        )
        .await;
        match serde_json::from_str::<DataRespAuthnLoginProcessEmail>(&resp_string) {
            Err(_err) => {
                msg_authentication_failed("Error parse after fetch DataRespAuthnLoginProcessEmail");
                return Err(anyhow!("error"));
            }
            Ok(resp_obj_email) => {
                let salt = resp_obj_email.salt;
                debug_write(&format!("response salt: {}", salt));
                return Ok(salt);
            }
        }
    }
    // send user_email and hash to server authn_login_process_hash and get the login_success
    async fn send_email_hash_get_authn_success(
        user_email: String,
        hash: String,
    ) -> anyhow::Result<bool> {
        let json_body = serde_json::to_string(&common_code::DataReqAuthnLoginProcessHash {
            user_email: user_email.clone(),
            hash,
        })
        .unwrap();
        let resp_string = fetch_json_response(
            format!("/{APP_MAIN_ROUTE}/{SCOPE}/authn_login_process_hash"),
            json_body,
        )
        .await;
        // If success then response contains the session cookie and is automatically saved to memory.
        // This cookie will be sent on every subsequent request until we close the browser. Then it is removed.
        match serde_json::from_str::<DataRespAuthnLoginProcessHash>(&resp_string) {
            Err(_err) => {
                msg_authentication_failed(" Error parse after fetch DataRespAuthnLoginProcessHash");
                return Err(anyhow!("error"));
            }
            Ok(resp_obj_hash) => {
                let login_success = resp_obj_hash.login_success;
                debug_write(&format!("response login success: {login_success}",));
                Ok(login_success)
            }
        }
    }

    fn msg_authentication_failed(debug_text: &str) {
        debug_write(debug_text);
        let div_alert = get_html_element_by_id("div_alert");
        div_alert.set_text_content(Some("Authentication failed !"));
    }
}
