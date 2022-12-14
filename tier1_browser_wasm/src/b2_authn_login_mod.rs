// tier1_browser_wasm/src/b2_authn_login_mod.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use tier0_common_code as T_0;

// For the on_click macro, I must use crate::on_click and wasm_bindgen::JsCast
use crate::on_click;
use crate::on_input;
use crate::web_sys_mod::*;
use T_0::APP_MAIN_ROUTE;

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
    if user_email.is_empty()
        || !user_email.contains("@")
        || !user_email.contains(".")
        || password.is_empty()
    {
        msg_div_alert_and_debug(
            "Authentication failed !",
            "email or password empty or incorrect",
        );
        return ();
    }

    let Ok(resp1_obj) = send_obj_get_obj::<T_0::DataRespAuthnLoginProcessEmail>(
        SCOPE,
        "b2_authn_login_process_email",
        T_0::DataReqAuthnLoginProcessEmail {
            user_email: user_email.clone(),
        },
    )
    .await else
    {
            msg_div_alert_and_debug_1("Authentication failed !");
            return ();
    };

    let password_hash = calculate_hash(user_email.clone(), password, resp1_obj.salt);

    let Ok(resp2_obj) = send_obj_get_obj::<T_0::DataRespAuthnLoginProcessHash>(
        SCOPE,
        "b2_authn_login_process_hash",
        T_0::DataReqAuthnLoginProcessHash {
            user_email: user_email.clone(),
            password_hash,
        },
    )
    .await else
    {
        msg_div_alert_and_debug_1("Authentication failed !");
        return ();
    };

    if resp2_obj.login_success == false {
        msg_div_alert_and_debug_1("Authentication failed !");
        return ();
    }
    window()
        .open_with_url_and_target(
            &format!("/{APP_MAIN_ROUTE}/b5_start_page_mod/b5_start_page"),
            "_self",
        )
        .unwrap();
}

pub fn calculate_hash(user_email: String, password: String, salt: String) -> String {
    let input_password = format!("{user_email}_{password}");
    let argon2 = argon2::Argon2::default();
    let password_hash =
        argon2::PasswordHasher::hash_password(&argon2, &input_password.as_bytes(), &salt)
            .unwrap()
            .to_string();
    password_hash
}
