// tier1_browser_wasm/src/a4_system_mod.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// For the on_click macro, I must use crate::on_click and wasm_bindgen::JsCast
use crate::on_click;
use crate::on_input;
use crate::web_sys_mod::*;

#[wasm_bindgen]
pub fn a4_string_encrypt_decrypt_on_start() {
    debug_write("a4_string_encrypt_decrypt_on_start");

    // region: add event listeners
    on_click!("btn_encrypt", a4_string_encrypt_btn);
    on_click!("btn_decrypt", a4_string_decrypt_btn);
    on_click!(
        "btn_generate_new_secret_key",
        a4_generate_new_secret_key_btn
    );
    on_input!("secret_key", a4_string_encrypt_reset_alert);
    on_input!("normal_string", a4_string_encrypt_reset_alert);
    on_input!("encrypted_string", a4_string_encrypt_reset_alert);
    // endregion: add event listeners
}

/// on key input
pub async fn a4_string_encrypt_reset_alert() {
    let div_alert = get_html_element_by_id("div_alert");
    div_alert.set_text_content(None);
}

/// on encrypt button click
pub async fn a4_string_encrypt_btn() {
    debug_write("a4_string_encrypt_btn");
    let secret_key = get_input_element_value_string_by_id("secret_key");
    let normal_string = get_input_element_value_string_by_id("normal_string");

    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };

    let Ok(secret_key_byte) = hex::decode(&secret_key) else {
            msg_div_alert_and_debug_1( "Secret_key is not 32 bytes hex!" );
            return ();
    };
    let key = aes_gcm::aead::generic_array::GenericArray::from_slice(&secret_key_byte);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let Ok(cipher_text) = cipher.encrypt(nonce, normal_string.as_bytes()) else {
        msg_div_alert_and_debug_1( "Encrypt error!" );
        return ();
    };
    let encrypted_string = hex::encode(&cipher_text);

    set_input_element_value_string_by_id("encrypted_string", &encrypted_string)
}

/// on decrypt button click
pub async fn a4_string_decrypt_btn() {
    debug_write("a4_string_decrypt_btn");
    let secret_key = get_input_element_value_string_by_id("secret_key");
    let encrypted_string = get_input_element_value_string_by_id("encrypted_string");

    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    let Ok(secret_key_byte) = hex::decode(&secret_key) else {
        msg_div_alert_and_debug_1( "Secret_key is not 32 bytes hex!" );
        return ();
};
    let Ok(encrypted_string_byte) = hex::decode(&encrypted_string) else {
        msg_div_alert_and_debug_1( "Encrypted_string is not in hex!" );
        return ();
};
    let key = aes_gcm::aead::generic_array::GenericArray::from_slice(&secret_key_byte);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let Ok(plaintext) = cipher.decrypt(nonce, encrypted_string_byte.as_ref()) else {
            msg_div_alert_and_debug_1( "Decrypt error!" );
            return ();
        };

    let Ok(normal_string) = String::from_utf8(plaintext) else {
        msg_div_alert_and_debug_1( "Decrypted string is not utf8!" );
        return ();
    };
    set_input_element_value_string_by_id("normal_string", &normal_string)
}

/// generate_new_secret_key button click
pub async fn a4_generate_new_secret_key_btn() {
    debug_write("a4_generate_new_secret_key_btn");
    use aes_gcm::aead::{KeyInit, OsRng};
    let key = aes_gcm::Aes256Gcm::generate_key(&mut OsRng);
    let rand_key256 = hex::encode(key);
    set_input_element_value_string_by_id("secret_key", &rand_key256)
}
