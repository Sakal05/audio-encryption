use crate::utils::{
    denormalize,
    limit_decimal_places,
    normalize,
    prevent_overflow,
    Function,
    KeyParameter,
};

use crate::decryption::decrypt;

pub fn encrypt(raw: Vec<u8>, c1: f64, c2: f64, y1: f64, y2: f64) -> Vec<u64> {
    // println!("Raw data: {:?}", raw.first());
    let mut encrypted_data: Vec<u64> = Vec::new();
    let mut raw_en: Vec<f64> = Vec::new();
    // println!("Y1: {y1} and Y2: {y2}");
    let mut y1_: f64 = y1;
    let mut y2_: f64 = y2;

    // Round p and q values outside the loop
    let rounded_c1_y1 = limit_decimal_places(c1 * y1_, 5);
    let rounded_c2_y2 = limit_decimal_places(c2 * y2_, 5);

    for byte in raw {
        let key_parameter = KeyParameter {
            x: normalize(byte as f64),
            p: rounded_c1_y1,
            q: rounded_c2_y2,
        };
        // println!("Before y function: {:?}", y);
        let mut y = key_parameter.y_function();
        prevent_overflow(&mut y);
        y = denormalize(y);
        // // let origin = y;
        raw_en.push(y);

        if y.is_finite() {
            y2_ = y1_;
            y1_ = y;
            // prevent_overflow(&mut y);
            encrypted_data.push(y.round() as u64);
        } else {
            // Handle the case where y is infinite
            println!("Y value is infinite! Skipping this byte.");
        }
        // encrypted_data.push(y.round() as u64);
    }

    let decrypted_data = decrypt(encrypted_data.clone(), c1, c2, y1, y2);

    for (p, mut byte) in encrypted_data.clone().into_iter().map(|f| {f as u8}).enumerate()  {
        if byte != decrypted_data[p] {
            byte += byte - decrypted_data[p];
        }
    }
    encrypted_data
}
