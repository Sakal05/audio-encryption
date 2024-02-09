use crate::utils::{
    denormalize, limit_decimal_places, normalize, prevent_overflow, read_waver, write_file, write_waver, Function, KeyParameter
};

use crate::decryption::decrypt;

pub fn encrypt(raw: Vec<f32>, c1: f32, c2: f32, y1: f32, y2: f32) -> Vec<f32> {
    // println!("Raw data: {:?}", raw.first());
    let mut encrypted_data: Vec<f32> = Vec::new();
    let mut raw_en: Vec<f32> = Vec::new();
    // println!("Y1: {y1} and Y2: {y2}");
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;

    // Round p and q values outside the loop
    let rounded_c1_y1 = limit_decimal_places(c1 * y1_, 5);
    let rounded_c2_y2 = limit_decimal_places(c2 * y2_, 5);

    for byte in raw.clone() {
        // println!("Normalize: {:?}", normalize(byte as f32));
        let key_parameter = KeyParameter {
            // x: normalize(byte as f32),
            x: byte,
            p: rounded_c1_y1,
            q: rounded_c2_y2,
        };
        // println!("Before y function: {:?}", y);
        let mut y = key_parameter.y_function();
        // prevent_overflow(&mut y);
        // y = normalize(y);

        if y.is_finite() {
            y2_ = y1_;
            y1_ = y;
            // raw_en.push(y);
            // prevent_overflow(&mut y);
            // println!("Before rounding and conversion: {}", y);
            let encrypted_byte = y;
            // println!("Rounded: {}", y.round() as u8);
            // if (encrypted_byte > 255) {
            //     println!("U8 max: {}", u8::MAX);
            //     println!("Too large");
            //     break;
            // }
            encrypted_data.push(encrypted_byte as f32);
        } else {
            // Handle the case where y is infinite
            println!("Y value is infinite! Skipping this byte.");
        }
        // encrypted_data.push(y.round() as u64);
    }
    let write_en_back_wave = write_waver("test_waveer_en.wav", encrypted_data.clone(), 44100, 2);
    let read_wa = read_waver("test_waveer_en.wav");
    let decrypted_data = decrypt(read_wa.bytes, c1, c2, y1, y2);
    let write_back_wave = write_waver("test_waveer_dec.wav", decrypted_data.clone(), 44100, 2);
    println!("Dec last bytes: {:?}", decrypted_data[1]);
    println!("En last bytes: {:?}", encrypted_data[1]);
    for (p, byte) in raw.clone().into_iter().enumerate() {
        if byte != decrypted_data[p] {
            println!("Diff: {}", byte - decrypted_data[p]);
            println!("Diff en: {}", byte - encrypted_data[p]);
        }
    }
 
    encrypted_data
}
