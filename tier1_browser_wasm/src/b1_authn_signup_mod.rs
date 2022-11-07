// tier1_browser_wasm/src/b1_authn_signup_mod.rs

// For the on_click macro, I must use crate::on_click and wasm_bindgen::JsCast
use crate::on_click;
use crate::on_input;
use crate::web_sys_mod::*;
// use t0::APP_MAIN_ROUTE;
use tier0_common_code as t0;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const SCOPE: &'static str = "b1_authn_signup_mod";

#[wasm_bindgen]
pub fn b1_authn_signup_on_start() {
    debug_write("b1_authn_signup_on_start");

    // region: add event listeners
    on_click!("btn_submit", b1_authn_signup_btn_submit);
    on_input!("user_email", b1_authn_signup_reset_alert);
    on_input!("password_1", b1_authn_signup_reset_alert);
    on_input!("password_2", b1_authn_signup_reset_alert);
    // endregion: add event listeners
}

/// on key input
pub async fn b1_authn_signup_reset_alert() {
    let div_alert = get_html_element_by_id("div_alert");
    div_alert.set_text_content(None);
}

/// on button click
pub async fn b1_authn_signup_btn_submit() {
    debug_write("b1_authn_signup_btn_submit");
    let user_email = get_input_element_value_string_by_id("user_email");
    let password_1 = get_input_element_value_string_by_id("password_1");
    let password_2 = get_input_element_value_string_by_id("password_2");
    if user_email.is_empty()
        || !user_email.contains("@")
        || password_1.is_empty()
        || password_2.is_empty()
        || password_1 != password_2
    {
        msg_div_alert_and_debug("Signup failed !", "email or password empty or incorrect");
        return ();
    }

    match send_obj_get_obj::<t0::DataRespAuthnSignupProcessEmail>(
        SCOPE,
        "b1_authn_signup_process_email",
        t0::DataReqAuthnSignupProcessEmail {
            user_email: user_email.clone(),
        },
    )
    .await
    {
        Err(_err) => {
            msg_div_alert_and_debug("Signup failed !", "Signup failed !");
            return ();
        }
        Ok(resp1_obj) => {
            
            let hash = crate::b2_authn_login_mod::calculate_hash(
                user_email.clone(),
                password_1,
                resp1_obj.salt,
            );
            
        }
    }
}
