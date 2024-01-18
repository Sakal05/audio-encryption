
mod key_generator;
mod encryption;
mod decryption;
use crate::encryption::encrypt;
use crate::decryption::decrypt;
pub mod utils;
use crate::utils::{read_file, write_file};

fn main() {
    let keys = "testing123xza1sd";
    let k_y1 = 0.341548792;
    let k_y2 = 0.123443282;

    let _test_key = key_generator::generate_key(keys, 0.23453412312, 0.6712345331, k_y1, k_y2).unwrap();
    println!("Key generator: {} {}", _test_key.0, _test_key.1);
    // let metadata = file.metadata().unwrap();
    let y1_prime = 0.3416785121;
    let y2_prime = 0.1987234512;
 
    let file_data = read_file("typing.mp3");
    println!("last byte of raw data: {:?}", file_data.last());
    let encrypted_data = encrypt(file_data, _test_key.0, _test_key.1, y1_prime, y2_prime);

    // // Open the output file for writing the encrypted data
    write_file("test_encrypted_typing.mp3", encrypted_data);    
    
    let en_file_data = read_file("test_encrypted_typing.mp3");
    println!("last byte of encrypted data: {:?}", en_file_data.last());
    
    // println!("De buffer: {:?}", de_buffer);
    let decrypted_data = decrypt(en_file_data, _test_key.0, _test_key.1, y1_prime, y2_prime);
    println!("last byte of decrypted data: {:?}", decrypted_data.last());
    // // Write the decrypted data to the output file
    write_file("test_decrypted_typing.mp3", decrypted_data);    

}