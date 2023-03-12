use std::borrow::Cow;
use std::path::Path;

use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;

#[proc_macro]
pub fn generate_disasembler(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
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
    let filename = Path::new(file.value());
    let filename = if filename.is_absolute() {
        Cow::Borrowed(filename)
    } else {
        let build_path =
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "".into());
        Cow::Owned(Path::new(&build_path).join(&filename))
    };
    let sleigh = sleigh2rust::parse_disassembler(&filename).unwrap();
    sleigh.into_token_stream().into()
}
