pub trait Function<T> {
    fn y_function(&self) -> f64;
    fn reverse_y_function(&self) -> f64;
}

#[derive(Debug)]
pub struct KeyParameter<T> {
    pub x: T,
    pub p: f64,
    pub q: f64
}


impl<T> Function<T> for KeyParameter<T> where
T: Into<f64> + Copy,
{
    fn y_function(&self) -> f64 {
        let x_as_f64: f64 = self.x.into(); // Convert character to a numeric type first
        // print all value
        // println!("x: {}, p: {}, q: {}", x_as_f64, self.p, self.q);
        let result = x_as_f64 + self.p + self.q; // Perform the arithmetic and implicitly return the result
        // println!("F Function result: {}", result);
        result
    }

    fn reverse_y_function(&self) -> f64 {
        let x_as_f64: f64 = self.x.into(); // Convert character to a numeric type first
        // print all value
        // println!("x: {}, p: {}, q: {}", x_as_f64, self.p, self.q);
        let result = x_as_f64 - self.p - self.q; // Perform the arithmetic and implicitly return the result
        // println!("F Function result: {}", result);
        result
    }
}

pub fn prevent_overflow(y: &mut f64) {
    *y = (*y + 1.00 % 2.00) - 1.00;
    // *y = y.clamp(0.0, 1.0);

}

const MAX_ORIGINAL: u8 = u8::MAX;
const MIN_ORIGINAL: u8 = u8::MIN;

pub fn normalize(n:f64) -> f64 {
    // (n - MIN_ORIGINAL as f64) / (MAX_ORIGINAL as f64 - MIN_ORIGINAL as f64)
    (n - 127.5) / 127.5
    // (n / 127.5) - 1.0
    // let n = (n - 128.0) / 128.0;
    // n
}

pub fn denormalize(n: f64) -> f64 {
    (n * 127.5) + 127.5
    // n * (MAX_ORIGINAL as f64 - MIN_ORIGINAL as f64) + MIN_ORIGINAL as f64
    // (n + 1.0) * 127.5
    // let n = (n * 128.0) + 128.0;
    // n
}

use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};

pub fn read_file(file_path: &str) -> Vec<u8> {
    let file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut buffer).unwrap();

    buffer
}

pub fn write_file(file_path: &str, bytes: Vec<u8>){
    let mut file =  BufWriter::new(File::create(file_path).unwrap());
    file.write_all(&bytes).unwrap();
}

pub fn limit_decimal_places(value: f64, decimal_places: usize) -> f64 {
    let multiplier = 10u64.pow(decimal_places as u32) as f64;
    (value * multiplier).round() / multiplier
}