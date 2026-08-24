mod lexer;

pub mod token;

#[derive(Default)]
pub struct Lexer<'src> {
    // source string to be lexed
    source: &'src [u8],

    // index of which the lexer is currently pointing to
    current: usize,
    // start index of the current token
    start: usize,
    // line number of the current token
    line: usize,
    // current column the lexer is pointing at
    column: usize,
    // index of where the column started for the current token
    start_column: usize,
}
