pub trait Function<T> {
    fn y_function(&self) -> f64;
    fn reverse_y_function(&self) -> f64;
}

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

pub fn prevent_overflow(y: f64) -> f64 {
    let of = (y % (2 as f64)) - (1 as f64);
    of
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