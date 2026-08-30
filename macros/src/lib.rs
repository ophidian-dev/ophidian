mod constructor;

use proc_macro::TokenStream;

#[proc_macro_derive(Constructor)]
pub fn constructor(input: TokenStream) -> TokenStream {
    constructor::expand(input)
}
