use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use syn::{
    parse_quote, parse_quote_spanned,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Semi},
    Attribute, Data, DataEnum, DataStruct, Error, Expr, ExprAssign, ExprLit, Field,
    FieldMutability, FieldValue, Fields, FieldsNamed, Index, Lit, LitFloat, Member, Meta, MetaList,
    MetaNameValue, Result, Token, Type, TypeTuple, Visibility, WhereClause, WherePredicate,
};

use crate::{
    match_arms::Arms,
    util::{find_custom_factor, TokenResult, VariantValues},
};

pub(crate) struct SurpriseFactorImpl {
    pub(crate) vis: Visibility,
    pub(crate) fields: Fields,
    pub(crate) semi_token: Option<Semi>,
    pub(crate) distribution_body: TokenStream,
    pub(crate) default_struct: TokenStream,
    pub(crate) default_assigns: Punctuated<ExprAssign, Semi>,
    pub(crate) default_where_clause: Option<WhereClause>,
}

impl SurpriseFactorImpl {
    pub(crate) fn new(
        vis: Visibility,
        name: &Ident,
        where_clause: Option<WhereClause>,
        data: Data,
    ) -> Result<Self> {
        let where_clause = where_clause.unwrap_or_else(|| WhereClause {
            where_token: Default::default(),
            predicates: Default::default(),
        });

        match data {
            Data::Struct(data) => Self::new_for_struct(vis, name, where_clause, data),
            Data::Enum(data) => Self::new_for_enum(vis, name, where_clause, data),
            Data::Union(data) => Err(Error::new_spanned(
                data.union_token,
                "Cannot derive `Surprise` for unions",
            )),
        }
    }

    fn new_for_struct(
        vis: Visibility,
        name: &Ident,
        mut where_clause: WhereClause,
        mut data: DataStruct,
    ) -> Result<Self> {
        let distribution_body = match data.fields {
            Fields::Named(ref fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| {
                        let name = &field
                            .ident
                            .as_ref()
                            .expect("missing field name on named fields");

                        let ty = &field.ty;

                        let tokens = if let Some(factor_name) = find_custom_factor(&field.attrs)? {
                            quote! {
                                #name: <
                                    #factor_name as ::surprise_me::Distribution<#ty>
                                >::sample(&self. #name, rng)
                            }
                        } else {
                            quote! {
                                #name: <
                                    <#ty as Surprise>::Factor
                                    as ::surprise_me::Distribution<#ty>
                                >::sample(&self. #name, rng)
                            }
                        };

                        Ok(tokens)
                    })
                    .map(TokenResult::from);

                quote! {
                    #[allow(clippy::needless_borrow)]
                    #name { #( #fields ,)* }
                }
            }
            Fields::Unnamed(ref fields) => {
                let fields = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let tuple_idx = Index {
                            index: i as u32,
                            span: field.span(),
                        };

                        let ty = &field.ty;

                        let tokens = if let Some(factor_name) = find_custom_factor(&field.attrs)? {
                            quote! {
                                <#factor_name as ::surprise_me::Distribution<#ty>>
                                    ::sample(&self. #tuple_idx, rng)
                            }
                        } else {
                            quote! {
                                <<#ty as Surprise>::Factor as ::surprise_me::Distribution<#ty>>
                                    ::sample(&self. #tuple_idx, rng)
                            }
                        };

                        Ok(tokens)
                    })
                    .map(TokenResult::from);

                quote! {
                    #[allow(clippy::needless_borrow)]
                    #name ( #( #fields ),* )
                }
            }
            Fields::Unit => quote!(#name),
        };

        let default_struct = match data.fields {
            Fields::Named(ref fields) => {
                let fields = fields.named.iter().map(|field| field.ident.as_ref());

                quote! {
                    Self {
                        #( #fields: Default::default(), )*
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let fields = fields.unnamed.iter().map(|_| quote!(Default::default()));

                quote! {
                    Self( #( #fields, )* )
                }
            }
            Fields::Unit => quote!(Self),
        };

        let mut default_assigns = Punctuated::new();

        match data.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    let member = Member::Named(field.ident.clone().unwrap());
                    let mut field_chain = vec![member];
                    parse_default_attrs(&field.attrs, &mut field_chain, &mut default_assigns)?;
                }
            }
            Fields::Unnamed(ref fields) => {
                for (i, field) in fields.unnamed.iter().enumerate() {
                    let idx = Index {
                        index: i as u32,
                        span: field.span(),
                    };
                    let member = Member::Unnamed(idx);
                    let mut field_chain = vec![member];
                    parse_default_attrs(&field.attrs, &mut field_chain, &mut default_assigns)?;
                }
            }
            Fields::Unit => {}
        }

        if !default_assigns.is_empty() {
            default_assigns.push_punct(Default::default());
        }

        for field in data.fields.iter_mut() {
            field.vis = Visibility::Public(Default::default());

            let ty = if let Some(factor_name) = find_custom_factor(&field.attrs)? {
                parse_quote!(#factor_name)
            } else {
                let ty = &field.ty;

                parse_quote!(<#ty as Surprise>::Factor)
            };

            where_clause
                .predicates
                .push(parse_quote!(#ty: ::std::default::Default));

            field.ty = ty;

            field.attrs.clear();

            if let Some(field_name) = field.ident.as_ref() {
                let field_doc =
                    format!(" The surprise factor counterpart of [`{name}::{field_name}`]");

                field.attrs.push(parse_quote!(#[doc = #field_doc]));
            }
        }

        Ok(Self {
            semi_token: data.semi_token,
            vis,
            fields: data.fields,
            distribution_body,
            default_struct,
            default_assigns,
            default_where_clause: (!where_clause.predicates.is_empty()).then_some(where_clause),
        })
    }

    fn new_for_enum(
        vis: Visibility,
        name: &Ident,
        mut where_clause: WhereClause,
        data: DataEnum,
    ) -> Result<Self> {
        // Fields of the new `{TypeName}Surprise` struct
        let mut named_fields = Punctuated::<Field, Comma>::new();

        let variants_weight_doc =
            format!(" The surprise factor weights for each variant of [`{name}`]");

        let variant_count = data.variants.len();

        named_fields.push_value(Field {
            attrs: vec![parse_quote_spanned! { name.span() => #[doc = #variants_weight_doc] }],
            vis: Visibility::Public(Default::default()),
            mutability: FieldMutability::None,
            ident: Some(Ident::new("variants_weight", name.span())),
            colon_token: Some(Token![:](name.span())),
            ty: parse_quote_spanned! { name.span() => Box<[f64; #variant_count]> },
        });

        // Match arms of the `Distribution::sample` method
        let mut factor_match_arms = Arms::default();

        // Fields of the Default implementation
        let mut default_assigns = Punctuated::<ExprAssign, Semi>::new();
        let mut variants_weight = Punctuated::<Expr, Comma>::new();
        let mut found_non_zero_weight = false;
        let mut found_weight_attr = false;

        for (i, variant) in data.variants.iter().enumerate() {
            let weight_lit = variant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("weight"))
                .map(|attr| match attr.meta {
                    Meta::NameValue(ref meta) => {
                        if let Expr::Lit(ref lit_expr) = meta.value {
                            match &lit_expr.lit {
                                Lit::Float(lit) => {
                                    let num = lit.base10_parse::<f64>()?;
                                    found_non_zero_weight |= num != 0.0;
                                    found_weight_attr = true;

                                    Ok(lit.to_owned())
                                }
                                Lit::Int(lit) => {
                                    let num = lit.base10_parse::<i64>()?;
                                    found_non_zero_weight |= num != 0;
                                    found_weight_attr = true;
                                    let num_str = format!("{:?}", num as f64);

                                    Ok(LitFloat::new(num_str.as_str(), lit.span()))
                                }
                                lit => Err(Error::new_spanned(lit, "expected number literal")),
                            }
                        } else {
                            Err(Error::new_spanned(&meta.value, "expected literal"))
                        }
                    }
                    Meta::Path(_) | Meta::List(_) => {
                        let msg = "expected `#[weight = literal]`";

                        Err(Error::new_spanned(&attr.meta, msg))
                    }
                })
                .transpose()?
                .unwrap_or_else(|| LitFloat::new("0.0", Span::call_site()));

            let variant_weight = Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Float(weight_lit),
            });

            variants_weight.push(variant_weight);

            let variant_name = &variant.ident;
            let i = Literal::usize_unsuffixed(i);

            match variant.fields {
                Fields::Named(ref fields) => {
                    if fields.named.is_empty() {
                        let match_arm = parse_quote_spanned! {
                            variant.span() => #i => #name :: #variant_name {}
                        };

                        factor_match_arms.push(match_arm);

                        continue;
                    }

                    let values = fields
                        .named
                        .iter()
                        .enumerate()
                        .map(|(j, field)| {
                            let field_span = field.span();
                            let field_name = field.ident.clone();
                            let ty = &field.ty;

                            let idx = Index {
                                index: j as u32,
                                span: field_span,
                            };

                            let tokens: FieldValue =
                                if let Some(factor_name) = find_custom_factor(&field.attrs)? {
                                    parse_quote_spanned! { ty.span() =>
                                        #field_name: <
                                            #factor_name as ::surprise_me::Distribution<#ty>
                                        >::sample(&self. #variant_name . #idx, rng)
                                    }
                                } else {
                                    parse_quote_spanned! { ty.span() =>
                                        #field_name: <
                                            <#ty as Surprise>::Factor
                                            as ::surprise_me::Distribution<#ty>
                                        >::sample(&self. #variant_name . #idx, rng)
                                    }
                                };

                            Ok(tokens)
                        })
                        .collect::<Result<_>>()?;

                    let variant_values = VariantValues::Fields(values);

                    let match_arm = parse_quote_spanned! { variant.span() =>
                        #i => #name :: #variant_name #variant_values
                    };

                    factor_match_arms.push(match_arm);
                }
                Fields::Unnamed(ref fields) => {
                    if fields.unnamed.is_empty() {
                        let match_arm = parse_quote_spanned! {
                            variant.span() => #i => #name :: #variant_name ()
                        };

                        factor_match_arms.push(match_arm);

                        continue;
                    }

                    let values = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(j, field)| {
                            let idx = Index {
                                index: j as u32,
                                span: field.span(),
                            };

                            let ty = &field.ty;

                            let tokens: Expr =
                                if let Some(factor_name) = find_custom_factor(&field.attrs)? {
                                    parse_quote_spanned! { field.span() =>
                                        <#factor_name as ::surprise_me::Distribution<#ty>>
                                            ::sample(&self. #variant_name . #idx, rng)
                                    }
                                } else {
                                    parse_quote_spanned! { field.span() =>
                                        <<#ty as Surprise>::Factor as
                                            ::surprise_me::Distribution<#ty>
                                        >::sample(&self. #variant_name . #idx, rng)
                                    }
                                };

                            Ok(tokens)
                        })
                        .collect::<Result<_>>()?;

                    let variant_values = VariantValues::Tuple(values);

                    let match_arm = parse_quote_spanned! { variant.span() =>
                        #i => #name :: #variant_name #variant_values
                    };

                    factor_match_arms.push(match_arm);
                }
                Fields::Unit => {
                    let match_arm = parse_quote_spanned! {
                        variant.span() => #i => #name :: #variant_name
                    };

                    factor_match_arms.push(match_arm);

                    continue;
                }
            }

            let mut field_tuple_elems = Punctuated::<Type, Comma>::new();

            let mut field_chain = vec![Member::Named(variant_name.to_owned())];

            for (i, field) in variant.fields.iter().enumerate() {
                let ty_elem = if let Some(factor_name) = find_custom_factor(&field.attrs)? {
                    parse_quote!(#factor_name)
                } else {
                    let ty = &field.ty;

                    parse_quote_spanned!(ty.span() => <#ty as Surprise>::Factor)
                };

                field_tuple_elems.push(ty_elem);

                let idx = Index {
                    index: i as u32,
                    span: field.span(),
                };

                field_chain.push(Member::Unnamed(idx));
                parse_default_attrs(&field.attrs, &mut field_chain, &mut default_assigns)?;
                field_chain.pop();
            }

            if field_tuple_elems.len() == 1 {
                field_tuple_elems.push_punct(Token![,](variant.span()));
            }

            let predicate_iter = field_tuple_elems.iter().map(|ty| -> WherePredicate {
                parse_quote_spanned! {
                    ty.span() => #ty: ::std::default::Default
                }
            });

            where_clause.predicates.extend(predicate_iter);

            let field_doc =
                format!(" The surprise factor counterpart to fields of [`{name}::{variant_name}`]");

            let named_field = Field {
                attrs: vec![parse_quote!(#[doc = #field_doc])],
                vis: Visibility::Public(Default::default()),
                mutability: FieldMutability::None,
                ident: Some(variant_name.to_owned()),
                colon_token: Some(Token![:](variant.span())),
                ty: Type::Tuple(TypeTuple {
                    paren_token: Default::default(),
                    elems: field_tuple_elems,
                }),
            };

            named_fields.push(named_field);
        }

        if !found_weight_attr {
            for lit in variants_weight.iter_mut() {
                *lit = parse_quote!(1.0);
            }
        } else if !found_non_zero_weight {
            let msg = "At least one variant must be denoted with a weight greater 0 i.e. `#[weight = num]`";

            return Err(Error::new(Span::call_site(), msg));
        }

        let default_fields = named_fields.iter().skip(1).map(|field| {
            let name = &field.ident;

            quote!(#name: Default::default())
        });

        let default_struct = quote! {
            Self {
                variants_weight: Box::new([ #variants_weight ]),
                #( #default_fields ,)*
            }
        };

        let named_fields = FieldsNamed {
            brace_token: Default::default(),
            named: named_fields,
        };

        let distribution_body = quote! {
            let weights = self.variants_weight.as_ref();
            let weighted_idx = ::surprise_me::rand::distributions::WeightedIndex::new(weights).unwrap();
            let idx = rng.sample(weighted_idx);

            #[allow(clippy::needless_borrow)]
            match idx {
                #factor_match_arms
            }
        };

        if !default_assigns.is_empty() {
            default_assigns.push_punct(Token![;](Span::call_site()));
        }

        Ok(Self {
            semi_token: None,
            vis,
            fields: Fields::Named(named_fields),
            distribution_body,
            default_struct,
            default_assigns,
            default_where_clause: (!where_clause.predicates.is_empty()).then_some(where_clause),
        })
    }
}

fn parse_default_attrs(
    attrs: &[Attribute],
    field_chain: &mut Vec<Member>,
    assigns: &mut Punctuated<ExprAssign, Semi>,
) -> Result<()> {
    fn parse_nested_attr(
        list: &MetaList,
        assigns: &mut Punctuated<ExprAssign, Semi>,
        field_chain: &mut Vec<Member>,
    ) -> Result<()> {
        let nested = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for meta in nested {
            match meta {
                Meta::Path(path) => {
                    let assign = parse_quote! {
                        default. #( #field_chain ).* = #path
                    };

                    assigns.push(assign);
                }
                Meta::NameValue(value) => {
                    let MetaNameValue { path, value, .. } = value;

                    let assign = parse_quote! {
                        default. #( #field_chain .)* #path = #value
                    };

                    assigns.push(assign);
                }
                Meta::List(inner_list) => {
                    let member = Member::Named(inner_list.path.get_ident().unwrap().clone());
                    field_chain.push(member);
                    parse_nested_attr(&inner_list, assigns, field_chain)?;
                    field_chain.pop();
                }
            }
        }

        Ok(())
    }

    for attr in attrs {
        if !attr.path().is_ident("factor") {
            continue;
        }

        match attr.meta {
            Meta::List(ref list) => parse_nested_attr(list, assigns, field_chain)?,
            Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(_), ..
                    }),
                ..
            }) => {}
            _ => {
                let msg = "expected `#[factor(field_name = literal[, ...])]` \
                    or `#[factor = \"SurpriseFactorType\"]`";

                return Err(Error::new_spanned(attr, msg));
            }
        }
    }

    Ok(())
}
