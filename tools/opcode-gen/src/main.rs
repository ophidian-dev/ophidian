use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Spec {
    opcodes: HashMap<String, u8>,
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 4 {
        eprintln!("Expected opcode spec file path, path to c opcode file and path to rust opcode file");
        std::process::exit(1);
    }

    let file: Result<String, std::io::Error> = fs::read_to_string(&argv[1]);
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

    let mut file: Result<fs::File, std::io::Error> = fs::File::create(&argv[2]);
    let mut c_file: fs::File = match file {
        Ok(f) => {
            f
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let file: Result<fs::File, std::io::Error> = fs::File::create(&argv[3]);
    let mut rust_file: fs::File = match file {
        Ok(f) => {
            f
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    write(&mut c_file, "// AUTO_GENERATED FROM spec/opcodes.toml\n");
    write(&mut c_file, "// DO NOT MANUALLY EDIT\n");

    for (key, value) in spec.opcodes {
        write(&mut c_file, "#define OP_");
        write(&mut c_file, &key.to_uppercase());
        write(&mut c_file, " ");
        write(&mut c_file, &format!("{}\n", value));

        
    }
}

fn write(file: &mut fs::File, text: &str) {
    if let Err(e) = write!(file, "{}", text) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

