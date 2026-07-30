use crate::lexer::token::TokenStream;

pub struct Parser<T>
where
    T: TokenStream,
{
    tokens: T,
}
