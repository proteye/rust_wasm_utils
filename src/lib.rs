mod utils;

extern crate wasm_bindgen;

use base64::prelude::*;
use std::str::from_utf8;
use utils::crypto::{decrypt, encrypt};
use utils::image::resize_image;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen(js_name = aesEncrypt)]
pub fn aes256_encrypt(key: &str, plaintext: &str) -> Vec<u8> {
    match wasm_log::try_init(wasm_log::Config::default()) {
        Ok(_) => {}
        Err(_) => {}
    }

    let encrypted = match encrypt(key, plaintext.as_bytes()) {
        Ok(encrypted) => encrypted,
        Err(e) => {
            println!("Encryption error: {}", e);
            return Vec::<u8>::new();
        }
    };

    let cipher_message = BASE64_STANDARD.encode(&encrypted);
    log::info!("ENCRYPTED: {}", cipher_message);

    return encrypted;
}

#[wasm_bindgen(js_name = aesDecrypt)]
pub fn aes256_decrypt(key: &str, ciphertext: &[u8]) -> String {
    match wasm_log::try_init(wasm_log::Config::default()) {
        Ok(_) => {}
        Err(_) => {}
    }

    let decrypted = match decrypt(key, ciphertext) {
        Ok(decrypted) => decrypted,
        Err(e) => {
            println!("Decryption error: {}", e);
            return "".to_string();
        }
    };

    let decrypted_message = from_utf8(decrypted.as_slice()).unwrap();
    log::info!("DECRYPTED: {}", decrypted_message);

    return decrypted_message.to_string();
}

#[wasm_bindgen(js_name = imageResize)]
pub fn image_resize(data: Vec<u8>, dst_width: u32, dst_height: u32) -> Vec<u8> {
    match wasm_log::try_init(wasm_log::Config::default()) {
        Ok(_) => {}
        Err(_) => {}
    }

    let result = resize_image(data, dst_width, dst_height);

    return result;
}
