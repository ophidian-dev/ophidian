use owo_colors::OwoColorize;
use op::lex::lexer::Lexer;

fn main() {

    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        eprintln!("{}: {} {}", argv[0], "error:".bright_red().bold(), "no input file".bold());
        std::process::exit(1);
    }

    let bytes: Result<Vec<u8>, std::io::Error> = std::fs::read(&argv[1]);
    let res: Vec<u8>;
    match bytes {
        Ok(v) => {
            res = v; 
        }
        Err(e) => {
            let msg = match e.kind() {
                std::io::ErrorKind::NotFound => String::from("No such file or directory"),
                std::io::ErrorKind::PermissionDenied => String::from("Permission denied"),
                std::io::ErrorKind::IsADirectory => String::from("Is a directory"),
                _ => format!("{}", e)
            };
            eprintln!("{}: {} {}{}{}{}", argv[0], 
                        "error:".bright_red().bold(), msg.bold(), 
                        ": '".bold(), argv[1].bold(), "'".bold());     
            std::process::exit(1);
        }
    }

    
    println!("{} Behaviour is undefined if file contains non-ASCII characters", "N.B.".yellow());

    // DEBUG THE LEXER
    let lexer = Lexer::new(&res);
    for token in lexer {
        println!("{:?}", token);
    }

}
