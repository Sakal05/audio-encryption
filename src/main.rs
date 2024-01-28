
// mod key_generator;
// mod encryption;
// mod decryption;
// use crate::encryption::encrypt;
// use crate::decryption::decrypt;
// pub mod utils;


use audioencrypt::{
    utils::{read_file, write_file},
    key_generator,
    encryption::encrypt,
    decryption::decrypt,
};

fn main() {
    let keys = "testing123xza1sd";
    let k_y1: f64 = 0.345567;
    let k_y2: f64 = 0.1545678;
    let c1: f64 = -0.34553456;
    let c2: f64 = 0.378675;

    let _test_key = key_generator::generate_key(keys, c1, c2, k_y1, k_y2).unwrap();
    println!("Key generator: {} {}", _test_key.0, _test_key.1);
    // let metadata = file.metadata().unwrap();
    let y1_prime = 0.38586416;
    let y2_prime = 0.1958887;
 
    let file_data = read_file("typing.mp3");
    println!("last byte of raw data: {:?}", file_data.first());
    let encrypted_data = encrypt(file_data, _test_key.0, _test_key.1, y1_prime, y2_prime);

    // // Open the output file for writing the encrypted data
    write_file("test_encrypted_typing.mp3", encrypted_data.clone().into_iter().map(|f| {f as u8}).collect());    

    let en_file_data = read_file("test_encrypted_typing.mp3");
    println!("last byte of encrypted data: {:?}", encrypted_data.last());
    // print!("Raw en: {:?}", encrypted_data);
    // println!("De buffer: {:?}", de_buffer);
    //encrypted_data.into_iter().map(|f| {f as u64}).collect()
    let decrypted_data = decrypt(encrypted_data, _test_key.0, _test_key.1, y1_prime, y2_prime);
    println!("last byte of decrypted data: {:?}", decrypted_data.first());
    // // // Write the decrypted data to the output file
    write_file("test_decrypted_typing.mp3", decrypted_data);    

}