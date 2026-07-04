use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Spec {
    opcodes: HashMap<String, u8>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected opcode spec file path");
        std::process::exit(1);
    }

    let file: Result<String, std::io::Error> = std::fs::read_to_string(&args[1]);
    let text: String = match file {
        Ok(s) => {
            s
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let spec: Spec = toml::from_str(&text).unwrap();

    println!("{:?}", spec.opcodes);
}