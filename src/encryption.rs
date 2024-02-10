use crate::utils::{
    prevent_overflow, read_waver, write_file, write_waver, Function, KeyParameter
};

use crate::decryption::decrypt;

pub fn encrypt(raw: Vec<f32>, c1: f32, c2: f32, y1: f32, y2: f32) -> Vec<f32> {
    let mut encrypted_data: Vec<f32> = Vec::new(); 
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;
    for byte in raw.clone() {
        // println!("Y1: {}, Y2: {}", y1_, y2_);
        let y = prevent_overflow(byte + (c1 * y1_) + (c2 * y2_));
        // println!("After y over function: {:?}", y);
        y2_ = y1_;
        y1_ = y;

        encrypted_data.push(y);
 
        if !y.is_finite() {
            println!("Infinite value is not finite! Skipping this byte.");
            break;
        }
        // encrypted_data.push(y.round() as u64);
    }

    // let dec = decrypt(encrypted_data.clone(), c1, c2, y1, y2);
    // println!("Last dec byte ==== : {:?}", dec.last());

    // for (p, byte) in raw.clone().into_iter().enumerate() {
    //     if byte == encrypted_data[p] {
    //         println!("same {}", p);
    //     } else {
    //         println!("Diff en: {}", byte - encrypted_data[p]);

    //     }
    // }
 
    encrypted_data
}
