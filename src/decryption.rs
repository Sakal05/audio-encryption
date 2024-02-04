use crate::utils::{ denormalize, limit_decimal_places, normalize, prevent_overflow, Function, KeyParameter };

pub fn decrypt(raw: Vec<u8>, c1: f64, c2: f64, y1: f64, y2: f64) -> Vec<u8> {
    let mut decrypted_data: Vec<u8> = Vec::new();
    let mut y1_: f64 = y1;
    let mut y2_: f64 = y2;

    let rounded_c1_y1 = limit_decimal_places(c1 * y1_, 5);
    let rounded_c2_y2 = limit_decimal_places(c2 * y2_, 5);

    for byte in raw {
        let key_parameter = KeyParameter {
            x: normalize(byte as f64),
            // x: byte as f64,
            p: rounded_c1_y1,
            q: rounded_c2_y2,
        };
        let mut y = key_parameter.reverse_y_function();
        // prevent_overflow(&mut y);
        y = denormalize(y);

        if y.is_finite() {
            y2_ = y1_;
            y1_ = y;
            // prevent_overflow(&mut y);
            decrypted_data.push(y.round() as u8);
        } else {
            // Handle the case where y is infinite
            println!("Y value is infinite! Skipping this byte.");
            // You might want to decide how to handle this scenario, e.g., skip the byte or use a default value.
        }
    }
    // println!("Decryption bytes: {:?}", decrypted_bytes.first());
    decrypted_data
}
