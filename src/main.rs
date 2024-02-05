use base64::prelude::*;
use rust_wasm_utils::utils::{decrypt, encrypt, resize};
use std::str::from_utf8;

extern crate rust_wasm_utils;

fn main() {
    // Image resize
    resize(
        "./data/nasa-4928x3279.png",
        "./data/nasa-1024x768.png",
        1024,
        768,
    );

    // Message encrypt and decrypt
    let key = b"abcdefghijklmnop";
    let plaintext = "Hello, world!";

    let ciphertext = match encrypt(key, plaintext.as_bytes()) {
        Ok(ciphertext) => ciphertext,
        Err(e) => {
            println!("Encryption error: {}", e);
            return;
        }
    };

    let decrypted = match decrypt(key, &ciphertext) {
        Ok(decrypted) => decrypted,
        Err(e) => {
            println!("Decryption error: {}", e);
            return;
        }
    };

    let cipher_message = BASE64_STANDARD.encode(&ciphertext);
    let decrypted_message = from_utf8(decrypted.as_slice()).unwrap();

    assert_eq!(cipher_message, "bBGDU5xv1L1+7hqe8rG6KQ==");
    assert_eq!(plaintext, decrypted_message);

    println!("Ciphertext: {:?}", cipher_message);
    println!("Decrypted: {:?}", decrypted_message);
}
