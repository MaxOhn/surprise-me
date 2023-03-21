use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Generics, Ident, Result};

use crate::{
    factor::SurpriseFactorImpl,
    util::{validate_attrs, CustomSurpriseFactor},
};

pub fn impl_surprise(input: DeriveInput) -> Result<TokenStream> {
    SurpriseImpl::new(input).map(|surprise_impl| quote!(#surprise_impl))
}

struct SurpriseImpl {
    name: Ident,
    factor_name: Ident,
    factor: Option<SurpriseFactorImpl>,
    generics: Generics,
}

impl SurpriseImpl {
    fn new(input: DeriveInput) -> Result<Self> {
        let DeriveInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        } = input;

        let this = if let CustomSurpriseFactor(Some(factor_name)) = validate_attrs(&attrs, &data)? {
            Self {
                name: ident,
                factor_name,
                factor: None,
                generics,
            }
        } else {
            let factor = SurpriseFactorImpl::new(vis, &ident, generics.where_clause.clone(), data)?;

            Self {
                factor: Some(factor),
                factor_name: format_ident!("{ident}Surprise"),
                name: ident,
                generics,
            }
        };

        Ok(this)
    }
}

impl ToTokens for SurpriseImpl {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let name = &self.name;
        let factor_name = &self.factor_name;

        let surprise = quote! {
            impl #impl_generics ::surprise_me::Surprise for #name #ty_generics #where_clause {
                type Factor = #factor_name #ty_generics;
            }
        };

        tokens.extend(surprise);

        if let Some(ref factor) = self.factor {
            let surprise_doc = format!(" A surprise factor for [`{name}`]");

            let SurpriseFactorImpl {
                vis,
                fields,
                semi_token,
                distribution_body,
                default_struct,
                default_assigns,
                default_where_clause,
            } = factor;

            let factor = quote! {
                #[allow(non_snake_case, clippy::type_complexity)]
                #[doc = #surprise_doc]
                #vis struct #factor_name #impl_generics #fields #semi_token

                impl #impl_generics ::surprise_me::rand::distributions::Distribution<#name #ty_generics> for #factor_name #ty_generics #where_clause {
                    #[inline]
                    fn sample<R: ::surprise_me::rand::Rng + ?Sized>(&self, rng: &mut R) -> #name #ty_generics {
                        #distribution_body
                    }
                }

                impl #impl_generics ::std::default::Default for #factor_name #ty_generics #default_where_clause {
                    #[inline]
                    fn default() -> Self {
                        let mut default = #default_struct;

                        #default_assigns

                        default
                    }
                }
            };

            tokens.extend(factor);
        }
    }
}
