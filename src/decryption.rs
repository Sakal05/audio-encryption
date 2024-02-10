use crate::utils::prevent_overflow;


// use crate::utils::{
//     Function, KeyParameter
// };
pub fn decrypt(raw: Vec<f32>, c1: f32, c2: f32, y1: f32, y2: f32) -> Vec<f32> {
    let mut decrypted_data: Vec<f32> = Vec::new();
    let mut y1_: f32 = y1;
    let mut y2_: f32 = y2;

    for byte in raw {
        // println!("Y1: {}, Y2: {}", y1_, y2_);
        let y = byte - (c1 * y1_) - (c2 * y2_);
        // let key_parameter = KeyParameter {
        //     x: byte,
        //     p: c1 * y1_,
        //     q: c2 * y2_,
        // };
        // let y = key_parameter.reverse_y_function();
        y2_ = y1_;
        y1_ = y;

        decrypted_data.push(y);

        if !y.is_finite() {
            println!("Infinite value is not finite! Skipping this byte.");
            break;
        }
    }

    decrypted_data
}
