use std::cmp;

use bae::FromAttributes;
use proc_macro::TokenStream;
use proc_macro2::{
    Span,
    TokenStream as TokenStream2,
};
use quote::quote;
use syn::punctuated::Punctuated;

type UnitFields = Punctuated<syn::Field, syn::Token![,]>;

struct Field {
    key: String,
    field: TokenStream2,
    callback: Option<syn::Path>,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl Eq for Field {}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

#[derive(FromAttributes)]
struct Pistachio {
    skip: Option<()>,
    flatten: Option<()>,
    rename: Option<syn::LitStr>,
    callback: Option<syn::Path>,
}

#[proc_macro_derive(Render, attributes(pistachio))]
pub fn derive_render(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::ItemStruct);
    let ident = &item.ident;
    let generics = &item.generics;
    let type_params = item.generics.type_params();
    let unit_fields = UnitFields::new();

    let mut errors = Vec::new();

    let fields = match item.fields {
        syn::Fields::Named(fields) => fields.named.into_iter(),
        syn::Fields::Unnamed(fields) => fields.unnamed.into_iter(),
        _ => unit_fields.into_iter(),
    };

    let mut flatten = Vec::new();
    let mut fields = fields
        .enumerate()
        .filter_map(|(index, field)| {
            let mut callback = None;
            let mut rename = None;
            let mut skip = false;

            match Pistachio::try_from_attributes(&field.attrs) {
                Ok(Some(pistachio)) => {
                    if pistachio.skip.is_some() {
                        skip = true;
                    }

                    if pistachio.flatten.is_some() {
                        flatten.push(field.ident.as_ref().map_or_else(
                            || {
                                let index = index.to_string();
                                let value = syn::LitInt::new(&index, Span::call_site());
                                quote!(#value)
                            },
                            |ident| quote!(#ident),
                        ));

                        skip = true;
                    }

                    if let Some(lit) = pistachio.rename {
                        rename = Some(lit.value());
                    }

                    if let Some(path) = pistachio.callback {
                        callback = Some(path);
                    }
                },
                Ok(None) => (),
                Err(err) => errors.push(err),
            };

            if skip {
                return None;
            }

            let (key, field) = field.ident.as_ref().map_or_else(
                || {
                    let index = index.to_string();
                    let value = syn::LitInt::new(&index, Span::call_site());
                    let key = rename.as_ref().cloned().unwrap_or(index);
                    (key, quote!(#value))
                },
                |ident| {
                    let key = rename
                        .as_ref()
                        .cloned()
                        .unwrap_or_else(|| ident.to_string());
                    (key, quote!(#ident))
                },
            );

            Some(Field {
                key,
                field,
                callback,
            })
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        let errors: Vec<_> = errors.into_iter().map(|e| e.to_compile_error()).collect();
        return quote! {
            fn _pistachio_derive_compile_errors() {
                #(#errors)*
            }
        }
        .into();
    }

    fields.sort_unstable();

    let render_field_escape = fields.iter().map(
        |Field {
             key,
             field,
             callback,
             ..
         }| {
            if let Some(callback) = callback {
                quote! {
                    #key => #callback(&self.#field, escape, writer).map(|_| true),
                }
            } else {
                quote! {
                    #key => self.#field.render_escape(escape, writer).map(|_| true),
                }
            }
        },
    );

    let render_field_section = fields.iter().map(|Field { key, field, .. }| {
        quote! {
            #key => self.#field.render_section(#key, context, writer).map(|_| true),
        }
    });

    let render_field_inverted_section = fields.iter().map(|Field { key, field, .. }| {
        quote! {
            #key => self.#field.render_inverted_section(#key, context, writer).map(|_| true),
        }
    });

    let flatten = &*flatten;
    let fields = fields.iter().map(|Field { field, .. }| field);

    let where_clause = type_params
        .map(|param| quote!(#param: ::pistachio::render::Render))
        .collect::<Vec<_>>();
    let where_clause = if !where_clause.is_empty() {
        quote!(where #(#where_clause),*)
    } else {
        quote!()
    };

    let tokens = quote! {
        impl #generics ::pistachio::render::Render for #ident #generics #where_clause {
            #[inline]
            fn size_hint(&self, template: &::pistachio::Template) -> usize {
                template.size_hint() #( + self.#fields.size_hint(template) )*
            }

            #[inline]
            fn render_section<S, W>(
                &self,
                key: &str,
                context: ::pistachio::render::Context<S>,
                writer: &mut W
            ) -> std::result::Result<(), ::pistachio::render::RenderError<W::Error>>
            where
                S: ::pistachio::render::RenderStack,
                W: ::pistachio::render::Writer,
            {
                let frame = ::pistachio::render::stack::Frame { name: key, data: self };

                context.push(&frame).render(writer)
            }

            #[inline]
            fn render_field_escape<W>(
                &self,
                key: &str,
                escape: ::pistachio::render::Escape,
                writer: &mut W
            ) -> std::result::Result<bool, ::pistachio::render::RenderError<W::Error>>
            where
                W: ::pistachio::render::Writer,
            {
                match key {
                    #( #render_field_escape )*
                    _ => Ok(
                        #( self.#flatten.render_field_escape(key, escape, writer)? ||)*
                        false
                    )
                }
            }

            #[inline]
            fn render_field_section<S, W>(
                &self,
                key: &str,
                context: ::pistachio::render::Context<S>,
                writer: &mut W
            ) -> std::result::Result<bool, ::pistachio::render::RenderError<W::Error>>
            where
                S: ::pistachio::render::RenderStack,
                W: ::pistachio::render::Writer,
            {
                match key {
                    #( #render_field_section )*
                    _ => Ok(
                        #( self.#flatten.render_field_section(key, context, writer)? ||)*
                        false
                    )
                }
            }

            #[inline]
            fn render_field_inverted_section<S, W>(
                &self,
                key: &str,
                context: ::pistachio::render::Context<S>,
                writer: &mut W
            ) -> std::result::Result<bool, ::pistachio::render::RenderError<W::Error>>
            where
                S: ::pistachio::render::RenderStack,
                W: ::pistachio::render::Writer,
            {
                match key {
                    #( #render_field_inverted_section )*
                    _ => Ok(
                        #( self.#flatten.render_field_inverted_section(key, context, writer)? ||)*
                        false
                    )
                }
            }
        }
    };

    TokenStream::from(tokens)
}
