// use std::io::Error;
use anyhow::Error;
use crate::utils::{Function, KeyParameter, prevent_overflow};

pub fn generate_key(key: &str, c1: f32, c2: f32, y1: f32, y2: f32) -> Result<(f32, f32), Error> {
    if key.len() != 16 {
        return Err(Error::msg("Invalid key length"));
    }

    // println!("All key: {:?}, C1: {}, C2: {}", key, c1, c2);
    let mut new_c1: f32 = 0 as f32;
    let mut new_c2: f32 = 0 as f32;
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;
    let mut _key = key.to_string();

    for _ in 0..3 {
        for c in _key.clone().chars() {
            let key_parameter = KeyParameter {
                x: c as u32 as f32,
                p: c1 * y1_,
                q: c2 * y2_,
            };
            let mut y = key_parameter.y_function();
            // println!("Before overlfow: {:?}", key_parameter);
            y = prevent_overflow(y);

            // println!("After overlfow: {:?}", y);
            y2_ = y1_;
            y1_ = y;
    
            new_c1 = y2_;
            new_c2 = y1_;
            
            // println!("y: {y1_}\ny': {y2_}\n===============");
        }
        let c1_xor_c2 = new_c1 as u64 ^ new_c2 as u64;
        let shifted_result = c1_xor_c2 << 3;
        _key = shifted_result.to_string();
    }
    // println!("Key C1: {new_c1}, Key C2: {new_c2}");

    Ok((new_c1, new_c2))
}