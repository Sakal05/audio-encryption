// use crate::utils::{ Function, KeyParameter };

use crate::utils::prevent_overflow;

pub fn encrypt(raw: Vec<f32>, c1: f32, c2: f32, y1: f32, y2: f32) -> Vec<f32> {
    let mut encrypted_data: Vec<f32> = Vec::new();
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;
    for byte in raw.clone() {
        // println!("Y1: {} and Y2: {}", y1_, y2_);
        let y = byte + (c1 * y1_) + (c2 * y2_);

        y2_ = y1_;
        y1_ = y;

        encrypted_data.push(y);

        if !y.is_finite() {
            println!("Infinite value is not finite! Skipping this byte.");
            break;
        }
    }
    println!("Encrypted data: {:?}", encrypted_data);
    // println!("Rae data: {:?}", raw);
    encrypted_data
}
