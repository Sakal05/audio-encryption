use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
mod key_generator;
fn main() {
    // Open the input file for reading
    // let file = File::open("typing.mp3").unwrap();
    // let metadata = file.metadata().unwrap();
    // // println!("Raw data: {:#?}", metadata);

    // // Read the audio data into a buffer
    // let mut buffer = Vec::new();
    // let mut reader = BufReader::new(file);
    // reader.read_to_end(&mut buffer).unwrap();

    // println!("Raw audio file: {:?}", buffer);
    let keys = "testing123xya1sd";

    let _test_key = key_generator::generate_key(keys, 0.23453412312, -0.6712345331, 0.341564532, -0.123453123).unwrap();

    // // Encrypt the audio data (simple XOR encryption for demonstration purposes)
    // let key: u8 = 0xAA; // Replace this with a more secure key in a real-world scenario
    // for byte in &mut buffer {
    //     *byte ^= key;
    // }

    // // Open the output file for writing the encrypted data
    // let mut encrypted_file = BufWriter::new(File::create("encrypted_typing.mp3").unwrap());

    // // Write the encrypted data to the output file
    // encrypted_file.write_all(&buffer).unwrap();
}
