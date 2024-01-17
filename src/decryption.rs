use crate::utils::{Function, KeyParameter, prevent_overflow};

pub fn decrypt(raw: Vec<u8>, c1: f64, c2: f64, y1: f64, y2: f64) -> Vec<u8>
{
    // println!("Raw data: {:?}", raw);
    let decrypted_bytes: Vec<u8> = raw.iter().map(|&byte| {
        // let mut new_c1: f64 = 0 as f64;
        // let mut new_c2: f64 = 0 as f64;
        let mut y1_: f64 = y1;
        let mut y2_: f64 = y2;

        let key_parameter = KeyParameter {
            x: byte as f64,
            p: c1 * y1_,
            q: c2 * y2_,
        };
        let mut y = key_parameter.y_function();
        // y = prevent_overflow(y);
        y2_ = y1_;
        y1_ = y;
        // if y.abs() < f64::EPSILON {
        //     y = 0.0;
        // }
        // new_c1 = y2_;
        // new_c2 = y1_;
        // println!("Decryption data: {:#?}", &y);
        return y as u8
    }).collect();
    println!("Decryption bytes: {:?}", decrypted_bytes.last());
    decrypted_bytes
}