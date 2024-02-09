use audioencrypt::{
    decryption::decrypt, encryption::encrypt, key_generator, utils::{ read_file, read_waver, write_file }
};
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    /*
    ======== Run Instruction ======
    Operation:
    - generate_key
    - encrypt
    - decrypt

    Arg 1: Operation
    Arg 2: Password 16 Char (generate_key) | file_name (encrypt and decrypt)
    Arg 3: C1 value
    Arg 4: C2 value
    Arg 5: Y1 value
    Arg 6: Y2 value

    Example run:
    Key Generation Example: cargo run -- operation=generate_key value=testing123xza1sd c1=0.345567 c2=0.134134 y1=-0.34553456 y2=0.378675
    Encryption: Example: cargo run -- operation=encrypt value=KWAN_Confuse.mp3 c1=-0.6812081080792964 c2=0.8923235548247561 y1=-0.34553456 y2=0.378675
    Decryption: Example: cargo run -- operation=decrypt value=encrypted_typing.mp3 c1=-0.6812081080792964 c2=0.8923235548247561 y1=-0.34553456 y2=0.378675
    */
    enum Operation {
        KeyGeneration,
        Encryption,
        Decryption,
    }
    let mut operation: Operation = Operation::KeyGeneration;
    let mut op_value: String = String::from("Password16chars");
    let mut c1: f32 = 0.123;
    let mut c2: f32 = 0.321;
    let mut y1: f32 = 0.654;
    let mut y2: f32 = 0.456;

    // let client = redis::Client::open("redis://127.0.0.1:6379").expect("Open Connection failed");
    // println!("Client: {:?}", client);
    // let mut con = client.get_connection().expect("Connection failed");

    if &args.len() < &6 {
        println!("Arguments must be exactly 6 arguments!\n Current Arg: {}", &args.len());
        return  
    }

    for arg in args.iter().skip(1) {
        let parts: Vec<&str> = arg.split('=').collect();
        // Check if the argument is in key=value format
        if parts.len() == 2 {
            let key = parts[0];
            let value = parts[1];

            match key {
                "operation" => {
                    match value {
                        "generate_key" => {
                            operation = Operation::KeyGeneration;
                            println!("Generating key...");
                        }
                        "encrypt" => {
                            operation = Operation::Encryption;
                            println!("Encrypting key...");
                        }
                        "decrypt" => {
                            operation = Operation::Decryption;
                            println!("Decrypting key...");
                        }
                        _ => {
                            println!("Operation not supported");
                            return
                        }
                    }
                }
                "value" => {
                    op_value = value.parse::<String>().expect("Invalid Value Parse");
                }
                "c1" => {
                    c1 = value.parse::<f32>().expect("Invalid C1 Parse");
                }
                "c2" => {
                    c2 = value.parse::<f32>().expect("Invalid C2 Parse");
                }
                "y1" => {
                    y1 = value.parse::<f32>().expect("Invalid Y1 Parse");
                }
                "y2" => {
                    y2 = value.parse::<f32>().expect("Invalid Y2 Parse");
                }
                _ => {
                    println!("Command not supported");
                    return
                }
            }
        }
    }

    let mut key_result: Option<(f32, f32)> = None;

    match operation {
        Operation::KeyGeneration => {
            key_result = Some(key_generator::generate_key(&op_value, c1, c2, y1, y2).expect("Invalid KeyGeneration"));
            // return key_result;
        }
        Operation::Encryption => {
            // let file_data = read_file(&op_value);
            let file_data3: audioencrypt::utils::AudioReadData = read_waver(&op_value);
            // print out file_data3.sample_rate, file_data3.n_channels
            println!("sample rate: {}, n_channel: {}", file_data3.sample_rate, file_data3.n_channels);
            // let write_back_wave = wrtie_waver("test_waveer.wav", file_data3.bytes.clone(), file_data3.sample_rate, file_data3.n_channels);
            // let _file_data2 = read_mp3(&op_value);
            // match _file_data2 {
            //     Some(data) => {
            //         println!("Data: {:?}", data);
            //         let _encrypted_data: Vec<f32> = encrypt(data, c1, c2, y1, y2);
            //     }
            //     None => println!("no data available")
            // }
            let _encrypted_data = encrypt(file_data3.bytes, c1, c2, y1, y2);
            // write_mp3();
            // println!("File data2: {:?}", _file_data2);
            // println!("last byte of raw data: {:?}", &_encrypted_data[1]);
            // println!("Last byte of encrypted data: {:?}", encrypted_data.first());
            // write_file(&format!("encrypted_{}", &op_value), encrypted_data);
        }
        Operation::Decryption => {
            // let file_data = read_file(&op_value);
            // // println!("last byte of encrypted data: {:?}", file_data.first());
            // let decrypted_data = decrypt(file_data, c1, c2, y1, y2);
            // println!("last decrypted data: {:?}", decrypted_data.first());
            // write_file(&format!("decrypted_{}", &op_value), decrypted_data);
        }
    }

    if let Some(result) = key_result {
        println!("Key Generation result: {} {}", result.0, result.1);
    }

}
