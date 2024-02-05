extern crate crypto;

use crypto::blockmodes;
use crypto::aes::{cbc_decryptor, cbc_encryptor, KeySize};
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};

use std::error::Error;

pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut encryptor = cbc_encryptor(KeySize::KeySize256, key, &[0; 16], blockmodes::PkcsPadding);

    let mut ciphertext = Vec::new();
    let mut read_buffer = RefReadBuffer::new(plaintext);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor
            .encrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();

        ciphertext.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(ciphertext)
}

pub fn decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decryptor = cbc_decryptor(KeySize::KeySize256, key, &[0; 16], blockmodes::PkcsPadding);

    let mut plaintext = Vec::new();
    let mut read_buffer = RefReadBuffer::new(ciphertext);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor
            .decrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();

        plaintext.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(plaintext)
}
