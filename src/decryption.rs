use crate::utils::{ denormalize, limit_decimal_places, normalize, prevent_overflow, read_file, Function, KeyParameter };

pub fn decrypt(raw: Vec<u8>, c1: f64, c2: f64, y1: f64, y2: f64) -> Vec<u8> {
    let mut decrypted_data: Vec<u8> = Vec::new();
    let mut y1_: f64 = y1;
    let mut y2_: f64 = y2;

    let rounded_c1_y1 = limit_decimal_places(c1 * y1_, 5);
    let rounded_c2_y2 = limit_decimal_places(c2 * y2_, 5);

    for byte in raw {
        let mut v: f64 = byte as f64;
        if byte == 255 {
            v = 256 as f64;
        }
        let key_parameter = KeyParameter {
            // x: normalize(byte as f64),
            x: v,
            p: rounded_c1_y1,
            q: rounded_c2_y2,
        };
        let mut y = key_parameter.reverse_y_function();
        // prevent_overflow(&mut y);
        // y = denormalize(y);

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


    let file_data = read_file("KWAN_Confuse.mp3");
    // for (p, byte) in decrypted_data.clone().into_iter().enumerate() {
    //     if byte == 255 {
    //         decrypted_data[p] = 255;
    //         println!("Value missed match: L {}, R {}", byte, decrypted_data[p]);
    //     }
    // }

    // for (p, byte) in file_data.clone().iter().enumerate() {
    //     if byte != &decrypted_data[p] {
    //         println!("Value missed match: Raw {}, Decrypted {}", byte, decrypted_data[p]);
    //         if(decrypted_data[p] == 254) {
    //             println!("test");
    //             // break;
    //         }
    //         decrypted_data[p] = 255;
    //     }
    // }
    // println!("Decryption bytes: {:?}", decrypted_bytes.first());
    decrypted_data
}
