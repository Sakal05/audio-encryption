// use std::io::Error;
use anyhow::Error;

// c1, c2, y1, x1, must be in the domain of [-1, 1]
pub fn generate_key(key: &str, c1: f64, c2: f64, y1: f64, y2: f64) -> Result<(), Error> {
    if key.len() != 16 {
        return Err(Error::msg("Invalid key length"));
    }

    println!("All key: {:?}, C1: {}, C2: {}", key, c1, c2);
    let mut new_c1: f64 = 0 as f64;
    let mut new_c2: f64 = 0 as f64;
    let mut y1_: f64 = y1;
    let mut y2_: f64 = y2;

    for c in key.chars() {
        // println!("{}: {}", i, c);
        let mut y = y_function(c, c1*y1_, c2*y2_);
        y = prevent_overflow(y);
        y2_ = y1_;
        // must be shifted three value first
        y1_ = y;

        new_c1 = y2_;
        new_c2 = y1_;
        println!("y: {y1_}\ny': {y2_}\n===============");
    }
    println!("C1: {new_c1}, C2: {new_c2}");

    Ok(())
}

fn y_function(x: char, p: f64, q: f64) -> f64 {
    let x_as_f64 = x as u32 as f64; // Convert character to a numeric type first
    // print all value
    println!("x: {}, p: {}, q: {}", x_as_f64, p, q);
    let result = x_as_f64 + p + q; // Perform the arithmetic and implicitly return the result
    println!("F Function result: {}", result);
    result
}

fn prevent_overflow(y: f64) -> f64 {
    let of = (y % (2 as f64)) - (1 as f64);
    of
}