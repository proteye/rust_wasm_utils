mod utils;

extern crate wasm_bindgen;

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

#[wasm_bindgen]
pub fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

#[wasm_bindgen(js_name = aesEncrypt)]
pub fn aes256_encrypt(key: &str, plaintext: &str) -> Vec<u8> {
    let encrypted = match encrypt(key.as_bytes(), plaintext.as_bytes()) {
        Ok(encrypted) => encrypted,
        Err(e) => {
            println!("Encryption error: {}", e);
            return Vec::<u8>::new();
        }
    };

    return encrypted;
}

#[wasm_bindgen(js_name = aesDecrypt)]
pub fn aes256_decrypt(key: &str, ciphertext: &[u8]) -> String {
    let decrypted = match decrypt(key.as_bytes(), ciphertext) {
        Ok(decrypted) => decrypted,
        Err(e) => {
            println!("Decryption error: {}", e);
            return "".to_string();
        }
    };
    let decrypted_message = from_utf8(decrypted.as_slice()).unwrap();

    return decrypted_message.to_string();
}

#[wasm_bindgen(js_name = imageResize)]
pub fn image_resize(data: &[u8], dst_width: u32, dst_height: u32) -> Vec<u8> {
    let result = resize_image(data, dst_width, dst_height);
    return result;
}
