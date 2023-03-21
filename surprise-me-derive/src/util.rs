use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    punctuated::Punctuated,
    token::{Brace, Comma, Paren},
    Attribute, Data, Error, Expr, ExprLit, Field, FieldValue, Fields, Lit, Meta, Result, Token,
};

pub(crate) struct CustomSurpriseFactor(pub(crate) Option<Ident>);

pub(crate) fn validate_attrs(attrs: &[Attribute], data: &Data) -> Result<CustomSurpriseFactor> {
    if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("weight")) {
        return Err(Error::new_spanned(attr, r#"expected `#[factor = "..."]`"#));
    }

    let factor = find_custom_factor(attrs)?;
    let has_factor = factor.is_some();

    enum AttrError<'a> {
        DoesNothing(&'a Attribute),
        OnlyFactor(&'a Attribute),
        NeedValueOrPath(&'a Attribute),
        NeedNameValue(&'a Attribute),
    }

    impl<O> From<AttrError<'_>> for Result<O> {
        #[inline]
        fn from(err: AttrError<'_>) -> Self {
            let err = match err {
                AttrError::DoesNothing(attr) => Error::new_spanned(
                    attr,
                    "Attribute does nothing when a custom surprise factor is specified",
                ),
                AttrError::OnlyFactor(attr) => {
                    Error::new_spanned(attr, "Only the `factor` attribute is allowed on fields")
                }
                AttrError::NeedValueOrPath(attr) => {
                    Error::new_spanned(attr, r#"expected `#[factor = "..."]` or `#[factor(...)]`"#)
                }
                AttrError::NeedNameValue(attr) => Error::new_spanned(
                    attr,
                    r#"expected `#[weight = number_literal]` or `#[factor = "TypeName"]`"#,
                ),
            };

            Err(err)
        }
    }

    fn check_fields(fields: &Punctuated<Field, Comma>, has_factor: bool) -> Result<()> {
        for field in fields {
            for attr in field.attrs.iter() {
                if has_factor {
                    return AttrError::DoesNothing(attr).into();
                } else if !attr.path().is_ident("factor") {
                    return AttrError::OnlyFactor(attr).into();
                }

                if let Meta::Path(_) = attr.meta {
                    return AttrError::NeedValueOrPath(attr).into();
                }
            }
        }

        Ok(())
    }

    match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(ref fields) => check_fields(&fields.named, has_factor)?,
            Fields::Unnamed(ref fields) => check_fields(&fields.unnamed, has_factor)?,
            Fields::Unit => {}
        },
        Data::Enum(data) => {
            for variant in data.variants.iter() {
                for attr in variant.attrs.iter() {
                    if let Meta::Path(_) | Meta::List(_) = attr.meta {
                        return AttrError::NeedNameValue(attr).into();
                    }
                }

                match variant.fields {
                    Fields::Named(ref fields) => check_fields(&fields.named, has_factor)?,
                    Fields::Unnamed(ref fields) => check_fields(&fields.unnamed, has_factor)?,
                    Fields::Unit => {}
                }
            }
        }
        Data::Union(_) => {}
    }

    Ok(CustomSurpriseFactor(factor))
}

pub(crate) fn find_custom_factor(attrs: &[Attribute]) -> Result<Option<Ident>> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("factor"))
        .find_map(|attr| {
            if let Meta::NameValue(ref name_value) = attr.meta {
                match &name_value.value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }) => Some(lit.parse()),
                    expr => {
                        let msg = r#"expected `#[factor = "TypeName"]`"#;

                        Some(Err(Error::new_spanned(expr, msg)))
                    }
                }
            } else {
                None
            }
        })
        .transpose()
}

pub(crate) enum TokenResult<T: ToTokens, E: ToTokens> {
    Ok(T),
    Err(E),
}

impl<T: ToTokens> From<Result<T>> for TokenResult<T, TokenStream> {
    #[inline]
    fn from(res: Result<T>) -> Self {
        match res {
            Ok(ok) => Self::Ok(ok),
            Err(err) => Self::Err(err.to_compile_error()),
        }
    }
}

impl<T: ToTokens, E: ToTokens> ToTokens for TokenResult<T, E> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TokenResult::Ok(ok) => ok.to_tokens(tokens),
            TokenResult::Err(err) => err.to_tokens(tokens),
        }
    }
}

pub(crate) enum VariantValues {
    Tuple(Punctuated<Expr, Comma>),
    Fields(Punctuated<FieldValue, Comma>),
}

impl ToTokens for VariantValues {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VariantValues::Tuple(values) => Paren::default().surround(tokens, |tokens| {
                tokens.append_separated(values, Token![,](Span::call_site()))
            }),
            VariantValues::Fields(values) => Brace::default().surround(tokens, |tokens| {
                tokens.append_separated(values, Token![,](Span::call_site()))
            }),
        }
    }
}
