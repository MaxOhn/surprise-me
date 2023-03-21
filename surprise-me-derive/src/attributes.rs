use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result,
};

#[derive(Default)]
pub(crate) struct Attributes {
    pub(crate) inner: Vec<Attribute>,
}

impl ToTokens for Attributes {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.inner);
    }
}

impl Parse for Attributes {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self> {
        input
            .call(Attribute::parse_outer)
            .map(|inner| Self { inner })
    }
}
