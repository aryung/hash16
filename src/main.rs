use hex;
use std::env;

fn encode_host16(name: &str) -> String {
    // Convert the input string to its hexadecimal representation
    let mut hex = hex::encode(name);

    // Insert dashes to format as a UUID
    let mut uuid_format = String::new();

    if hex.ends_with("0") {
        // If the hex string ends with a zero, append a specific pattern of 9's and 0's
        // Adjust this logic based on the actual pattern you need
        hex.push_str("999");
    }

    while hex.len() <= 32 {
        // If the hex string is less than 32 characters, append zeros
        hex.push('0');
    }

    let parts = [8, 4, 4, 4, 12]; // Positions for dashes in a UUID
    let mut current_pos = 0;

    for &part in parts.iter() {
        if current_pos != 0 {
            uuid_format.push('-');
        }
        if current_pos + part < hex.len() {
            uuid_format.push_str(&hex[current_pos..current_pos + part]);
        } else {
            uuid_format.push_str(&hex[current_pos..]);
            break;
        }
        current_pos += part;
    }

    uuid_format
}

fn decode_host16(encoded: &str) -> Result<String, hex::FromHexError> {
    // Remove dashes to get a continuous hex string
    let clean_encoded = encoded.replace("-", "");

    let processed_encoded = if let Some(pos) = clean_encoded.find("999") {
        // If "999" is found, remove "999" and any following zeros
        let (prefix, _) = clean_encoded.split_at(pos);
        String::from(prefix)
    } else {
        // If "999" is not found, but the string ends with zeros, remove trailing zeros
        clean_encoded.trim_end_matches('0').to_string()
    };

    // Decode from hexadecimal to bytes
    let hex_decoded = hex::decode(&processed_encoded)?;

    // Convert bytes back to UTF-8 string
    Ok(String::from_utf8_lossy(&hex_decoded).to_string())
}

fn main() {
    // let encoded_140 = encode_host16("CIRRUS-005140");
    // println!("Encoded CIRRUS-005140: {}", encoded_140);
    //
    // let encoded_143 = encode_host16("CIRRUS-005143");
    // println!("Encoded CIRRUS-005143: {}", encoded_143);
    //
    // let encoded_0143 = encode_host16("CIRRUS-5143");
    // println!("Encoded CIRRUS-5143: {}", encoded_0143);
    //
    // let encoded_cirrus = encode_host16("CIRRUS");
    // println!("Encoded CIRRUS: {}", encoded_cirrus);
    //
    // let examples = vec![
    //     "43495252-5553-2d30-3035-313430999900", // With "999" followed by zeros
    //     "43495252-5553-2d30-3035-313433000000", // Ending with zeros, no "999"
    // ];
    //
    // for encoded in examples {
    //     match decode_host16(encoded) {
    //         Ok(decoded) => println!("Decoded: {}", decoded),
    //         Err(e) => println!("Error decoding: {}", e),
    //     }
    // }

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} -h <name> or {} -p <encoded>", args[0], args[0]);
        std::process::exit(1);
    }

    let flag = &args[1];
    let input = &args[2];

    match flag.as_str() {
        "-h" => {
            let encoded = encode_host16(input);
            println!("{}", encoded);
        }
        "-p" => match decode_host16(input) {
            Ok(decoded) => println!("{}", decoded),
            Err(e) => {
                eprintln!("Failed to decode: {}", e);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Invalid flag '{}', use -h for encode, -p for decode", flag);
            std::process::exit(1);
        }
    }
}
