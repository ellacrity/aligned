use proc_macro2::TokenStream;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Decode;

impl Decode {
    pub fn impl_block(input: TokenStream) -> syn::Result<TokenStream> {
        Ok(input)
    }
}
