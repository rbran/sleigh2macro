use std::path::Path;

use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;

#[proc_macro]
pub fn parse(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: TokenStream = item.into();
    //one element is required
    let file: [TokenTree; 1] = item
        .into_iter()
        .collect::<Vec<TokenTree>>()
        .try_into()
        .unwrap();
    let file = match &file[0] {
        TokenTree::Literal(lit) => litrs::StringLit::try_from(lit).unwrap(),
        TokenTree::Group(_) | TokenTree::Ident(_) | TokenTree::Punct(_) => {
            panic!("Invalid parameter Type")
        }
    };
    let sleigh = sleigh2rust::parse(Path::new(file.value())).unwrap();
    sleigh.into_token_stream().into()
}
