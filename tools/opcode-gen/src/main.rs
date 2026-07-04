use serde::Deserialize;
use std::fs;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Spec {
    opcodes: Vec<Opcode>,
}

#[derive(Debug, Deserialize)]
struct Opcode {
    name: String,
    value: u8,
    description: String,
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 4 {
        eprintln!(
            "Expected opcode spec file path, path to c opcode file and path to rust opcode file"
        );
        std::process::exit(1);
    }

    let file: Result<String, std::io::Error> = fs::read_to_string(&argv[1]);
    let text: String = match file {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let spec: Spec = toml::from_str(&text).unwrap();

    let file: Result<fs::File, std::io::Error> = fs::File::create(&argv[2]);
    let mut c_file: fs::File = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let file: Result<fs::File, std::io::Error> = fs::File::create(&argv[3]);
    let mut rust_file: fs::File = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    write(&mut c_file, "// This file was auto-generated from spec/opcodes.toml\n");
    write(&mut c_file, "// It is not intended for manual editing\n\n");

    write(
        &mut rust_file,
        "// This file was auto-generated from spec/opcodes.toml\n",
    );
    write(&mut rust_file, "// It is not intended for manual editing\n\n");
    write(&mut rust_file, "#[repr(i32)]\n");
    write(&mut rust_file, "pub enum Opcode {\n\n");

    let tab: &str = "    ";

    for opcode in &spec.opcodes {
        let name = &opcode.name;
        let value = opcode.value;
        let description = &opcode.description;
        // c file
        write(&mut c_file, &format!("// {}\n", description));
        write(&mut c_file, "#define OP_");
        write(&mut c_file, &name.to_uppercase());
        write(&mut c_file, " ");
        write(&mut c_file, &format!("{}\n\n", value));

        // rust file
        write(&mut rust_file, tab);
        write(&mut rust_file, &format!("// {}\n", description));
        write(&mut rust_file, tab);
        write(&mut rust_file, &to_camel_case(&name));
        write(&mut rust_file, " = ");
        write(&mut rust_file, &format!("{}", value));
        write(&mut rust_file, ",\n");
    }

    write(&mut rust_file, "}\n");
}

fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
            continue;
        }

        if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.extend(c.to_lowercase());
        }
    }

    result
}

fn write(file: &mut fs::File, text: &str) {
    if let Err(e) = write!(file, "{}", text) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
