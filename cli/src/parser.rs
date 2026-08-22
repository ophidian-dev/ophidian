use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub input: String,

    // whether or not to dump bytecode
    // does not affect execution
    #[arg(short, long)]
    pub dump: bool,
}
