pub trait Function<T> {
    fn y_function(&self) -> f32;
    fn reverse_y_function(&self) -> f32;
}

pub struct KeyParameter<T> {
    pub x: T,
    pub p: f32,
    pub q: f32
}


impl<T> Function<T> for KeyParameter<T> where
T: Into<f32> + Copy,
{
    fn y_function(&self) -> f32 {
        let x_as_f32: f32 = self.x.into(); // Convert character to a numeric type first
        // print all value
        // println!("x: {}, p: {}, q: {}", x_as_f32, self.p, self.q);
        let result = x_as_f32 + self.p + self.q; // Perform the arithmetic and implicitly return the result
        // println!("F Function result: {}", result);
        result
    }

    fn reverse_y_function(&self) -> f32 {
        let x_as_f32: f32 = self.x.into(); // Convert character to a numeric type first
        // print all value
        // println!("x: {}, p: {}, q: {}", x_as_f32, self.p, self.q);
        let result = x_as_f32 - self.p - self.q; // Perform the arithmetic and implicitly return the result
        // println!("F Function result: {}", result);
        result
    }
}

pub fn prevent_overflow(y: f32) -> f32 {
    let of = (y % (2 as f32)) - (1 as f32);
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