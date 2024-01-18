use crate::utils::{Function, KeyParameter};

pub fn decrypt(raw: Vec<u8>, c1: f32, c2: f32, y1: f32, y2: f32) -> Vec<u8>
{
    let mut decrypted_data: Vec<u8> = Vec::new();
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;

    for byte in raw {
        let key_parameter = KeyParameter {
            x: byte as f32,
            p: c1 * y1_,
            q: c2 * y2_,
        };
        let y = key_parameter.reverse_y_function();
        // y = prevent_overflow(y);
        y2_ = y1_;
        y1_ = y;
        decrypted_data.push(y as u8);
    };
    // println!("Decryption bytes: {:?}", decrypted_bytes.first());
    decrypted_data
}