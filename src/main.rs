use owo_colors::OwoColorize;
use op::lex::lexer::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        eprintln!("{}: {} {}", argv[0], "error:".bright_red().bold(), "no input file".bold());
        std::process::exit(1);
    }

    println!("{} Behaviour is undefined if file contains non-ASCII characters", "N.B.".yellow());
    
    let bytes = std::fs::read(&argv[1])?;
    let lexer = Lexer::new(&bytes);
    for token in lexer {
        println!("{:?}", token);
    }

    Ok(())
}
