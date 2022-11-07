// tier1_browser_wasm/src/b2_authn_login_mod.rs

// For the on_click macro, I must use crate::on_click and wasm_bindgen::JsCast
use crate::on_click;
use crate::on_input;
use crate::web_sys_mod::*;
use t0::APP_MAIN_ROUTE;
use tier0_common_code as t0;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const SCOPE: &'static str = "b2_authn_login_mod";

#[wasm_bindgen]
pub fn b2_authn_login_on_start() {
    debug_write("b2_authn_login_on_start");

    // region: add event listeners
    on_click!("btn_submit", b2_authn_login_btn_submit);
    on_input!("user_email", b2_authn_login_reset_alert);
    on_input!("password", b2_authn_login_reset_alert);

    // endregion: add event listeners
}

/// on key input
pub async fn b2_authn_login_reset_alert() {
    let div_alert = get_html_element_by_id("div_alert");
    div_alert.set_text_content(None);
}

/// on button click
pub async fn b2_authn_login_btn_submit() {
    debug_write("b2_authn_login_btn_submit");
    let user_email = get_input_element_value_string_by_id("user_email");
    let password = get_input_element_value_string_by_id("password");
    if user_email.is_empty() || !user_email.contains("@") || password.is_empty() {
        msg_div_alert_and_debug(
            "Authentication failed !",
            "email or password empty or incorrect",
        );
        return ();
    }

    match send_obj_get_obj::<t0::DataRespAuthnLoginProcessEmail>(
        SCOPE,
        "b2_authn_login_process_email",
        t0::DataReqAuthnLoginProcessEmail {
            user_email: user_email.clone(),
        },
    )
    .await
    {
        Err(_err) => {
            msg_div_alert_and_debug("Authentication failed !", "Authentication failed !");
            return ();
        }
        Ok(resp1_obj) => {
            let hash = calculate_hash(user_email.clone(), password, resp1_obj.salt);

            match send_obj_get_obj::<t0::DataRespAuthnLoginProcessHash>(
                SCOPE,
                "b2_authn_login_process_hash",
                t0::DataReqAuthnLoginProcessHash {
                    user_email: user_email.clone(),
                    hash,
                },
            )
            .await
            {
                Err(_err) => {
                    msg_div_alert_and_debug("Authentication failed !", "Authentication failed !");
                    return ();
                }
                Ok(resp2_obj) => {
                    if resp2_obj.login_success == false {
                        msg_div_alert_and_debug(
                            "Authentication failed !",
                            "Authentication failed !",
                        );
                        return ();
                    } else {
                        window()
                            .open_with_url_and_target(
                                &format!(
                                    "/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list"
                                ),
                                "_self",
                            )
                            .unwrap();
                    }
                }
            }
        }
    }
}

pub fn calculate_hash(user_email: String, password: String, salt: String) -> String {
    let input_password = format!("{user_email}_{password}");
    debug_write(&input_password);
    let argon2 = argon2::Argon2::default();
    let hash = argon2::PasswordHasher::hash_password(&argon2, &input_password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    hash
}
