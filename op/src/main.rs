use compiler::Compiler;
use frontend::diagnostics::DiagnosticFormatter;
use owo_colors::OwoColorize;
use runtime::vm::VirtualMachine;

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

    let mut compiler = Compiler::new();
    let chunk = match compiler.compile(&file) {
        Ok(chunk) => chunk,
        Err(diags) => {
            let fmtter = DiagnosticFormatter::new(&file);
            for diag in &diags {
                fmtter.format(diag);
            }
            println!(
                "\n {} {} generated.",
                diags.len(),
                if diags.len() == 1 { "error" } else { "errors" }
            );
            std::process::exit(1);
        }
    };

    let mut vm = VirtualMachine::new();
    vm.execute(&chunk);
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
