use crate::utils::{ prevent_overflow, read_waver };

pub fn decrypt(raw: Vec<f32>, c1: f32, c2: f32, y1: f32, y2: f32) -> Vec<f32> {
    let mut decrypted_data: Vec<f32> = Vec::new();
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;

    for byte in raw {
        // println!("Y1: {}, Y2: {}", y1_, y2_);
        let y = prevent_overflow(byte - c1 * y1_ - c2 * y2_);

        y2_ = y1_;
        y1_ = y;
        // prevent_overflow(&mut y);
        decrypted_data.push(y);

        if !y.is_finite() {
            println!("Infinite value is not finite! Skipping this byte.");
            break;
        }
    }

    let file_data3 = read_waver("er.wav");

    // for (p, byte) in file_data3.bytes.clone().iter().enumerate() {
    //     let tolerance = 0.0001; // Adjust as needed

    //     // Compare the absolute difference between the values with the tolerance
    //     if (byte - &decrypted_data[p]).abs() > tolerance {
    //         println!("Mismatch at position {}: Raw {}, Decrypted {}, File {}",
    //                 p, *byte, decrypted_data[p], file_data3.bytes[p]);
    //     } else {
    //         println!("Same at position {}: Raw {}, Decrypted {}, File {}",
    //                 p, *byte, decrypted_data[p], file_data3.bytes[p]);
    //     }
    // }
    // Clone the decrypted_data vector for adjustment
    // let mut adjusted_data = decrypted_data.clone();

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
    decrypted_data
}
