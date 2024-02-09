use crate::utils::{
    denormalize,
    limit_decimal_places,
    normalize,
    prevent_overflow,
    read_file,
    Function,
    KeyParameter,
};

pub fn decrypt(raw: Vec<f32>, c1: f32, c2: f32, y1: f32, y2: f32) -> Vec<f32> {
    let mut decrypted_data: Vec<f32> = Vec::new();
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;

    let rounded_c1_y1 = limit_decimal_places(c1 * y1_, 5);
    let rounded_c2_y2 = limit_decimal_places(c2 * y2_, 5);

    for byte in raw {
        // let mut v: f32 = byte as f32;
        // if byte == 255 {
        //     v = 256 as f32;
        // }
        let key_parameter = KeyParameter {
            // x: normalize(byte as f32),
            x: byte,
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
            decrypted_data.push(y as f32);
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

    // Clone the decrypted_data vector for adjustment
    let mut adjusted_data = decrypted_data.clone();

    // Adjust decrypted_data values
    // Adjust decrypted_data values
    // for byte in adjusted_data.iter_mut() {
    //     if *byte == 255 {
    //         *byte = 25;
    //     }
    //     else if *byte == 254 {
    //         *byte = 255;
    //     }
    //     // println!(
    //     //     "Adjusted at position {}: Raw {}, Decrypted {}, File {}",
    //     //     p,
    //     //     *byte,
    //     //     decrypted_data[p],
    //     //     file_data[p]
    //     // );
    // }

    // // Compare the adjusted_data with file_data
    // for (p, byte) in adjusted_data.iter().enumerate() {
    //     if *byte != file_data[p] {
    //         println!(
    //             "Mismatch at position {}: Raw {}, Decrypted {}, File {}",
    //             p,
    //             *byte,
    //             decrypted_data[p],
    //             file_data[p]
    //         );
    //     }
    // }

    // // Now, you can use adjusted_data as needed
    // decrypted_data = adjusted_data;

    // // Compare decrypted_data with file_data
    // for (p, byte) in decrypted_data.iter().enumerate() {
    //     if byte != &file_data[p] {
    //         println!("Value2 missed match: Raw {}, Decrypted {}", byte, decrypted_data[p]);
    //         // Additional logic can be added here if needed
    //     } else {
    //         // The values are the same, so no need to print a message
    //         // Additional logic can be added here if needed
    //     }
    // }
    // println!("Decryption bytes: {:?}", decrypted_bytes.first());
    adjusted_data
}
