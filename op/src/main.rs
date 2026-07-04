use owo_colors::OwoColorize;
use frontend::lex::lexer::Lexer;
use frontend::parse::parser::Parser;

fn main() {

    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        eprintln!("{}: {} {}", argv[0], "error:".bright_red().bold(), "no input file".bold());
        std::process::exit(1);
    }

    let file = read_file_as_bytes(&argv[0], &argv[1]);

    println!("{} Behaviour is undefined if file contains non-ASCII characters", "N.B.".yellow());

    let lexer = Lexer::new(&file);
    let mut parser = Parser::new(lexer);
    let ast = parser.generate_ast();
    println!("{:?}", ast);
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
                _ => format!("{}", e)
            };
            eprintln!("{}: {} {}{}{}{}", invocation, 
                        "error:".bright_red().bold(), msg.bold(), 
                        ": '".bold(), file_name.bold(), "'".bold());     
            std::process::exit(1);
        }
    }
}