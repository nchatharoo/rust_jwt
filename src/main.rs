use clap::{Arg, Command};
use rust_jwt::{create_jwt, decode_jwt};
use jsonwebtoken::Algorithm;

fn main() {
    let matches = Command::new("Rust_jwt")
        .about("A simple app to encode and decode JWT")
        .version("1.0")
        .author("Your Name")
        .subcommand(
            Command::new("encode")
                .about("Encodes a JWT")
                .arg(Arg::new("subject").required(true).help("subject"))
                .arg(Arg::new("name").required(true).help("name"))
                .arg(Arg::new("secret").required(true).help("Secret key"))
                .arg(Arg::new("algorithm")
                    .required(true)
                    .help("Signing algorithm (HS256, HS384, HS512, RS256, RS384, RS512, ES256, ES384, EdDSA, PS256, PS512, PS384)")),
        )
        .subcommand(
            Command::new("decode")
                .about("Decodes a JWT")
                .arg(Arg::new("token").required(true).help("JWT to decode"))
                .arg(Arg::new("secret").required(true).help("Secret key"))
                .arg(Arg::new("algorithm")
                    .required(true)
                    .help("Signing algorithm (HS256, HS384, HS512, RS256, RS384, RS512, ES256, ES384, EdDSA, PS256, PS512, PS384)")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("encode") {
        let subject = matches.get_one::<String>("subject").unwrap();
        let name = matches.get_one::<String>("name").unwrap();
        let secret = matches.get_one::<String>("secret").unwrap();
        let algorithm_str = matches.get_one::<String>("algorithm").unwrap();

        let algorithm = match algorithm_str.as_str() {
            "HS256" => Algorithm::HS256,
            "HS384" => Algorithm::HS384,
            "HS512" => Algorithm::HS512,
            "RS256" => Algorithm::RS256, 
            "RS384" => Algorithm::RS384, 
            "RS512" => Algorithm::RS512, 
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            "EdDSA" => Algorithm::EdDSA, 
            "PS256" => Algorithm::PS256,
            "PS512" => Algorithm::PS512,
            "PS384" => Algorithm::PS384,
            _ => {
                eprintln!("Unsupported algorithm: {}", algorithm_str);
                return;
            }
        };

        match create_jwt(subject, name, secret, algorithm) {
            Ok(token) => println!("Generated JWT: {}", token),
            Err(err) => eprintln!("Error generating token: {}", err),
        }
    }

    if let Some(matches) = matches.subcommand_matches("decode") {
        let token = matches.get_one::<String>("token").unwrap();
        let secret = matches.get_one::<String>("secret").unwrap();
        let algorithm_str = matches.get_one::<String>("algorithm").unwrap();

        let algorithm = match algorithm_str.as_str() {
            "HS256" => Algorithm::HS256,
            "HS384" => Algorithm::HS384,
            "HS512" => Algorithm::HS512,
            "RS256" => Algorithm::RS256, 
            "RS384" => Algorithm::RS384, 
            "RS512" => Algorithm::RS512, 
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            "EdDSA" => Algorithm::EdDSA, 
            "PS256" => Algorithm::PS256,
            "PS512" => Algorithm::PS512,
            "PS384" => Algorithm::PS384,
            _ => {
                eprintln!("Unsupported algorithm: {}", algorithm_str);
                return;
            }
        };

        match decode_jwt(token, secret, algorithm) {
            Ok(data) => println!("Decoded claims: {:?}", data.claims),
            Err(err) => eprintln!("Error decoding token: {}", err),
        }
    }
}
