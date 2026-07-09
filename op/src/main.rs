use compiler::Compiler;
use compiler::bindings;
use frontend::diagnostics::DiagnosticEmitter;
use owo_colors::OwoColorize;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        eprintln!(
            "{}: {} {}",
            argv[0],
            "error:".bright_red().bold(),
            "no input file".bold()
        );
        std::process::exit(1);
    }

    let file = read_file_as_bytes(&argv[0], &argv[1]);

    println!(
        "{} Behaviour is undefined if file contains non-ASCII characters",
        "N.B.".yellow()
    );

    let mut compiler = Compiler::new();
    let res = compiler.compile(&file);

    let chunk = match res {
        Ok(c) => c,
        Err(diagnostics) => {
            let emitter = DiagnosticEmitter::new(&file);
            for diagnostic in &diagnostics {
                eprintln!("{}", emitter.emit(diagnostic));
            }

            println!(
                "\n{} error{} generated.",
                diagnostics.len(),
                plural(diagnostics.len())
            );

            std::process::exit(1);
        }
    };

    // println!("{:?}", chunk);
    // std::process::exit(1);

    unsafe {
        let bytecode_len = chunk.bytecode().len();
        let constant_len = chunk.constants().len();
        let (mut bytecode, mut constants) = chunk.chunk_data();
        let bytecode: *mut u8 = bytecode.as_mut_ptr();
        let constants: *mut bindings::vm_Value = constants.as_mut_ptr();

        bindings::vm_execute(bytecode, bytecode_len, constants, constant_len);
    }
}

fn plural(i: usize) -> &'static str {
    if i == 1 {
        return "";
    }
    "s"
}

fn read_file_as_bytes(invocation: &str, file_name: &str) -> Vec<u8> {
    let bytes: Result<Vec<u8>, std::io::Error> = std::fs::read(file_name);

    match bytes {
        Ok(v) => {
            return v;
        }
        Err(e) => {
            let msg = match e.kind() {
                std::io::ErrorKind::NotFound => String::from("No such file or directory"),
                std::io::ErrorKind::PermissionDenied => String::from("Permission denied"),
                std::io::ErrorKind::IsADirectory => String::from("Is a directory"),
                _ => format!("{}", e),
            };
            eprintln!(
                "{}: {} {}{}{}{}",
                invocation,
                "error:".bright_red().bold(),
                msg.bold(),
                ": '".bold(),
                file_name.bold(),
                "'".bold()
            );
            std::process::exit(1);
        }
    }
}
