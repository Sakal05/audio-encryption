use crate::utils::{Function, KeyParameter};

pub fn encrypt(raw: Vec<u8>, c1: f32, c2: f32, y1: f32, y2: f32) -> Vec<u8>
{
    // println!("Raw data: {:?}", raw.first());
    let mut encrypted_data: Vec<u8> = Vec::new();
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;

    for byte in raw {
        let key_parameter = KeyParameter {
            x: byte as f32,
            p: c1 * y1_,
            q: c2 * y2_,
        };
        let y = key_parameter.y_function();
        y2_ = y1_;
        y1_ = y;
        encrypted_data.push(y as u8);
    }
    // let encrypted_bytes: Vec<u8> = raw.iter().map(|&byte| {

    //     return y as u8
    // }).collect();

    encrypted_data
}