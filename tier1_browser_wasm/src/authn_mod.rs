// authn_mod.rs
// This module is for the SCOPE "/authn/". There is a folder with the same name with html files.
// Every html file has its separate sub-module.

// wasm sub-module for authn_login.html
pub mod authn_login {
    // For the on_click macro, I must use crate::on_click and wasm_bindgen::JsCast
    use crate::on_click;
    use crate::web_sys_mod::*;
    use crate::APP_MAIN_ROUTE;
    use common_code::ResultAuthnLoginProcessEmail;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    const SCOPE: &'static str = "authn";

    #[wasm_bindgen]
    pub fn authn_login_on_start() {
        debug_write("authn_login_on_start");

        // region: add event listeners
        on_click!("btn_submit", btn_submit)
        // endregion: add event listeners
    }

    pub async fn btn_submit() {
        debug_write("btn_submit");
        let user_email = get_input_element_value_string_by_id("user_email");
        let password = get_input_element_value_string_by_id("password");
        if user_email.is_empty() || password.is_empty() {
            let div_alert = get_html_element_by_id("div_alert");
            div_alert.set_text_content(Some("Authentication failed !"));
            return ();
        }
        // region: send user_email to server authn_login_process_email and expect the salt
        let resp_obj: common_code::ResultAuthnLoginProcessEmail = fetch_object_response(
            format!("/{APP_MAIN_ROUTE}/{SCOPE}/authn_login_process_email"),
            common_code::DataReqAuthnLoginProcessEmail {
                user_email: user_email.clone(),
            },
        )
        .await;

        match resp_obj {
            ResultAuthnLoginProcessEmail::Error(_err) => {
                let div_alert = get_html_element_by_id("div_alert");
                div_alert.set_text_content(Some("Authentication failed !"));
            }
            ResultAuthnLoginProcessEmail::Data { salt } => {
                debug_write(&format!("response salt: {}", salt));
                // endregion: send user_email to server authn_login_process_email and expect the salt

                // region: generate salted hash and send to server
                let input_password = format!("{user_email}_{password}");
                let argon2 = argon2::Argon2::default();
                let hash = argon2::PasswordHasher::hash_password(
                    &argon2,
                    &input_password.as_bytes(),
                    &salt,
                )
                .unwrap()
                .to_string();

                // send password hash to the server
                let resp_obj: common_code::DataRespAuthnLoginProcessHash = fetch_object_response(
                    format!("/{APP_MAIN_ROUTE}/{SCOPE}/authn_login_process_hash"),
                    common_code::DataReqAuthnLoginProcessHash {
                        user_email: user_email.clone(),
                        hash,
                    },
                )
                .await;
                let login_success = resp_obj.login_success;

                debug_write(&format!("response login success: {login_success}",));
                // endregion: generate salted hash and send to server

                if login_success == true {
                    window()
                        .open_with_url_and_target(
                            &format!("/{APP_MAIN_ROUTE}/webpage_hits/webpage_hits_list"),
                            "_self",
                        )
                        .unwrap();
                } else {
                    let div_alert = get_html_element_by_id("div_alert");
                    div_alert.set_text_content(Some("Authentication failed !"));
                    // TODO: on key_up for email or password empty the alert textContent 2022-10-21
                }
            }
        }
    }
}
