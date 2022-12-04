// tier1_browser_wasm/src/a4_system_mod.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// For the on_click macro, I must use crate::on_click and wasm_bindgen::JsCast
use crate::on_click;
use crate::web_sys_mod::*;
// use T_0::APP_MAIN_ROUTE;

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
    // endregion: add event listeners
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
    let secret_key_byte = hex::decode(&secret_key).unwrap();
    let key = aes_gcm::aead::generic_array::GenericArray::from_slice(&secret_key_byte);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let cipher_text = cipher.encrypt(nonce, normal_string.as_bytes()).unwrap();
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
    let secret_key_byte = hex::decode(&secret_key).unwrap();
    let encrypted_string_byte = hex::decode(&encrypted_string).unwrap();
    let key = aes_gcm::aead::generic_array::GenericArray::from_slice(&secret_key_byte);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let plaintext = cipher
        .decrypt(nonce, encrypted_string_byte.as_ref())
        .unwrap();

    let normal_string = String::from_utf8(plaintext).unwrap();
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
