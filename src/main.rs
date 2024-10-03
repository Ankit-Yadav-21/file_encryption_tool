mod utils;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("File Encryption Tool")
        .arg(Arg::new("mode").long("mode").value_parser(clap::value_parser!(String)).required(true))
        .arg(Arg::new("input").long("input").value_parser(clap::value_parser!(String)).required(true))
        .arg(Arg::new("output").long("output").value_parser(clap::value_parser!(String)).required(true))
        .arg(Arg::new("key").long("key").value_parser(clap::value_parser!(String)).required(false))
        .arg(Arg::new("generate_key").long("generate_key").action(clap::ArgAction::SetTrue))
        .get_matches();

    let mode = matches.get_one::<String>("mode").unwrap();
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    let key = if *matches.get_one::<bool>("generate_key").unwrap_or(&false) {
        utils::generate_key()
    } else if let Some(key_str) = matches.get_one::<String>("key") {
        utils::parse_key(key_str).expect("Failed to parse key")
    } else {
        panic!("Key not provided, and --generate_key not set");
    };

    match mode.as_str() {
        "encrypt" => {
            if let Err(e) = utils::encrypt_decrypt_file(input_file, output_file, &key) {
                eprintln!("Failed to encrypt file: {}", e);
            }
        }
        "decrypt" => {
            if let Err(e) =  utils::encrypt_decrypt_file(input_file, output_file, &key) {
                eprintln!("Failed to decrypt file: {}", e);
            }
        }
        _ => eprintln!("Invalid mode specified: {}", mode),
    }
}
