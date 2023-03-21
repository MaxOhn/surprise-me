use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{parse_quote, Arm, Token};

#[derive(Default)]
pub(crate) struct Arms {
    inner: Vec<Arm>,
}

impl Arms {
    pub(crate) fn push(&mut self, arm: Arm) {
        self.inner.push(arm);
    }
}

impl ToTokens for Arms {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_terminated(self.inner.iter(), Token![,](Span::call_site()));

        let wild: Arm = parse_quote! { _ => unreachable!() };
        wild.to_tokens(tokens);
    }
}
