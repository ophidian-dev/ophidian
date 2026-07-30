use crate::lexer::TokenStream;

pub struct Parser<T>
where
    T: TokenStream,
{
    tokens: T,
}
