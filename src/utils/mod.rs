extern crate crypto;
extern crate image;

use std::fs::File;
use std::io::BufWriter;
use std::num::NonZeroU32;

use crypto::blockmodes;
use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};

use crypto::aes::{cbc_decryptor, cbc_encryptor, KeySize};
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use fast_image_resize as fr;

use std::error::Error;

pub fn resize(src_path: &str, dst_path: &str, dst_width: u32, dst_height: u32) {
    // Read source image from file
    let img = ImageReader::open(src_path).unwrap().decode().unwrap();
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let mut src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )
    .unwrap();

    // Multiple RGB channels of source image by alpha channel
    // (not required for the Nearest algorithm)
    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();

    // Create container for data of destination image
    let dst_width = NonZeroU32::new(dst_width).unwrap();
    let dst_height = NonZeroU32::new(dst_height).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    // Write destination image as PNG-file
    let file = File::create(dst_path).unwrap();
    let mut result_buf = BufWriter::new(file);
    PngEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8,
        )
        .unwrap();
}

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
