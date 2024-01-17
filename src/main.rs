
mod key_generator;
mod encryption;
mod decryption;
use crate::encryption::encrypt;
use crate::decryption::decrypt;
pub mod utils;
use crate::utils::{read_file, write_file};

fn main() {
    let keys = "testing123xza1sd";

    let _test_key = key_generator::generate_key(keys, 0.23453412312, 0.6712345331, 0.341564532, 0.123453123).unwrap();
    // let metadata = file.metadata().unwrap();
 
    let file_data = read_file("typing.mp3");
    let _test_encrypt = encrypt(file_data, _test_key.0, _test_key.1, 0.341522532, 0.12345883);

    // // Open the output file for writing the encrypted data
    write_file("test_encrypted_typing.mp3", _test_encrypt);    
    
    let en_file_data = read_file("test_encrypted_typing.mp3");

    // println!("De buffer: {:?}", de_buffer);
    let _test_decrypt = decrypt(en_file_data, _test_key.0, _test_key.1, 0.341522532, 0.12345883);
    // // Write the decrypted data to the output file
    write_file("test_decrypted_typing.mp3", _test_decrypt);    

}