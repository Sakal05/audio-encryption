use crate::utils::{
    denormalize,
    limit_decimal_places,
    normalize,
    prevent_overflow,
    write_file,
    Function,
    KeyParameter,
};

use crate::decryption::decrypt;

pub fn encrypt(raw: Vec<u8>, c1: f64, c2: f64, y1: f64, y2: f64) -> Vec<u64> {
    // println!("Raw data: {:?}", raw.first());
    let mut encrypted_data: Vec<u64> = Vec::new();
    let mut raw_en: Vec<f64> = Vec::new();
    // println!("Y1: {y1} and Y2: {y2}");
    let mut y1_: f64 = y1;
    let mut y2_: f64 = y2;

    // Round p and q values outside the loop
    let rounded_c1_y1 = limit_decimal_places(c1 * y1_, 5);
    let rounded_c2_y2 = limit_decimal_places(c2 * y2_, 5);

    for byte in raw.clone() {
        // println!("Normalize: {:?}", normalize(byte as f64));
        let key_parameter = KeyParameter {
            x: normalize(byte as f64),
            // x: byte,
            p: rounded_c1_y1,
            q: rounded_c2_y2,
        };
        // println!("Before y function: {:?}", y);
        let mut y = key_parameter.y_function();
        // prevent_overflow(&mut y);
        // y = normalize(y);

        if y.is_finite() {
            y2_ = y1_;
            y1_ = y;
            // raw_en.push(y);
            // prevent_overflow(&mut y);
            // println!("Before rounding and conversion: {}", y);
            let encrypted_byte = y;
            // println!("Rounded: {}", y.round() as u8);
            // if (encrypted_byte > 255) {
            //     println!("U8 max: {}", u8::MAX);
            //     println!("Too large");
            //     break;
            // }
            encrypted_data.push(encrypted_byte.round() as u64);
        } else {
            // Handle the case where y is infinite
            println!("Y value is infinite! Skipping this byte.");
        }
        // encrypted_data.push(y.round() as u64);
    }

    let decrypted_data = decrypt(encrypted_data.clone(), c1, c2, y1, y2);
    println!("Dec last bytes: {:?}", decrypted_data[1]);
    // for (p, byte) in raw.clone().into_iter().enumerate() {
    //     if byte < decrypted_data[p] {
    //         // encrypted_data[p] -= 1;
    //     }
    //     // let adjusted: u8;
    //     // if byte != decrypted_data[p] {
    //     //     encrypted_data[p] = 25;
    //     // }
    //     //     println!("Raw BEfore encrypted data: {:?}", encrypted_data[p]);
    //     // if byte < decrypted_data[p] {
    //     //     adjusted = decrypted_data[p].wrapping_sub(byte);
    //     //     println!("Encrypted data: {:?}", encrypted_data[p]);
    //     //     // adjusted = decrypted_data[p] - byte;
    //     //     encrypted_data[p].wrapping_sub(adjusted);
    //     //     // encrypted_data[p] -= adjusted;
    //     //     println!("Smaller adjust: {}", adjusted);
    //     // } else if byte > decrypted_data[p] {
    //     //     println!("Encrypted data: {:?}", encrypted_data[p]);
    //     //     // adjusted = byte - decrypted_data[p];
    //     //     adjusted = byte.wrapping_sub(decrypted_data[p]);
    //     //     encrypted_data[p].wrapping_sub(adjusted);
    //     //     // encrypted_data[p] += adjusted;
    //     //     println!("Bigger adjust: {}", adjusted);
    //     // }
    //         // println!("Adjusted: {:?}", adjusted);
    //         // let after = encrypted_data[p].wrapping_add(adjusted);
    //         // println!("Raw After encrypted data: {:?}", encrypted_data[p]);
    //         // println!("After added: {:?}", after);
    //         // println!("Value missed match: L {}, R {}", byte, decrypted_data[p]);
    //     // }
    // }

    // let mut after_decrypted_data: Vec<u8> = decrypt(encrypted_data.clone(), c1, c2, y1, y2);
    // for (p, byte) in after_decrypted_data.clone().into_iter().enumerate() {
    //     // if raw[p] != byte {
    //         // println!("Raw: {:?} / De: {}", raw[p], byte);
    //         if byte == 255 {
    //             encrypted_data[p] -= 1;
    //             after_decrypted_data[p] -= 1;
    //             println!("Raw: {:?} / De: {} / En: {}", raw[p], after_decrypted_data[p], encrypted_data[p]);
    //         }
    //     // }
    // }
    // let finalafter_decrypted_data: Vec<u8> = decrypt(encrypted_data.clone(), c1, c2, y1, y2);
    // for (p, byte) in finalafter_decrypted_data.clone().into_iter().enumerate() {
    //     if raw[p] != byte {
    //         // println!("Raw: {:?} / De: {}", raw[p], byte);
    //         // if byte == 255 {
    //         //     encrypted_data[p] -= 1;
    //         //     after_decrypted_data[p] -= 1;
    //         //     println!("Raw: {:?} / De: {} / En: {}", raw[p], after_decrypted_data[p], encrypted_data[p]);
    //         // }
    //     }
    // }
    write_file(&format!("eeeeedsdfsadfe.mp3"), decrypted_data.clone());
    write_file(&format!("encyrpted_leng.mp3"), encrypted_data.clone().into_iter().map(|f| f as u8).collect());
    // println!("adjusted decrypted data: {:?}", byte.wrapping_sub(decrypted_byte as u64) as i64);
    // let mut finalized_encrypted_data: Vec<u8> = Vec::new();
    // for (p, byte) in encrypted_data.into_iter().enumerate() {
    //     let decrypted_byte = decrypted_data.get(p).map_or(0u8, |&b| b);
    //     if decrypted_byte != byte {
    //         let adjusted_byte = byte.wrapping_sub(decrypted_byte);
    //         // println!("Adjusted byte: {:?}", adjusted_byte);
    //         if adjusted_byte < 0 {
    //             finalized_encrypted_data.push(byte - adjusted_byte);
    //         } else {
    //             finalized_encrypted_data.push(byte.wrapping_add(adjusted_byte));

    //         }
    //     } else {
    //         println!("Exact correct: {}", decrypted_byte);
    //         finalized_encrypted_data.push(byte);
    //     }
    // }


    // finalized_encrypted_data
    encrypted_data
}
