// tier1_browser_wasm/lib.rs
// wasm module name: tier1_browser_wasm

use std::env;
//use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys_mod::*;

mod b1_authn_signup_mod;
mod b2_authn_login_mod;
mod web_sys_html_encode_mod;
mod web_sys_mod;

pub const APP_MAIN_ROUTE: &'static str = "webpage_hits_admin";

// I want to make this function available to javascript
pub use b2_authn_login_mod::authn_login_on_start;

/// To start the Wasm application, wasm_bindgen runs this function.
/// Wasm-bindgen creates a javascript wrapper around this function called init().
/// This is why we write `import init from "xxx.js"; init();` in the html script element.
#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    debug_write(&format!(
        "--- start tier1_browser_wasm ver. {} ---",
        env!("CARGO_PKG_VERSION")
    ));

    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    Ok(())
}
