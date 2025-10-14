use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Parsable, attributes(literal))]
pub fn parsable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !input.generics.params.empty_or_trailing() {
        return quote! { 
            compile_error!("`Parsable` cannot be derived for generic items") 
        }.into()
    }
    
    match input.data {
        syn::Data::Struct(data_struct) => struct_derive(input.ident, data_struct).into(),

        syn::Data::Enum(data_enum) => enum_derive(input.ident, data_enum).into(),
        
        syn::Data::Union(..) => quote! { 
            compile_error!("`Parsable` cannot be derived for unions") 
        }.into(),
    }
}

fn parse_item(ty: &syn::Type) -> TokenStream2 {
    fn derive_tokens(tokens: TokenStream2) -> TokenStream2 {
        quote! {
            <#tokens as leben_parsable::Parsable>::parse(stream)?
        }
    }

    match ty {
        #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]
        
        syn::Type::Macro(type_macro) => {
            derive_tokens(quote! { #type_macro }.into())},
        syn::Type::Paren(type_paren) => {
            parse_item(&type_paren.elem)},
        syn::Type::Path(type_path) => {
            derive_tokens(quote! { #type_path })},
        syn::Type::Tuple(type_tuple) => {
            let parses = type_tuple.elems.iter()
                .map(|ty| parse_item(ty));
            quote! { ( #( #parses ),* ) }
        },
        syn::Type::Array(type_array) => quote! { 
            compile_error!(format!("`Parsable` cannot be derived for array type {} (consider using `RepeatLimited`)", #type_array))},
        syn::Type::BareFn(type_bare_fn) => quote! { 
            compile_error!(format!("`Parsable` cannot be derived for bare function type {}", #type_bare_fn))},
        syn::Type::Group(type_group) => quote! {
            compile_error!(format!("`Parsable` cannot be derived for type group {}", #type_group))},
        syn::Type::ImplTrait(type_impl_trait) => quote! { 
            compile_error!(format!("`Parsable` cannot be derived for impl trait type {}", #type_impl_trait))},
        syn::Type::Infer(type_infer) => quote! {
            compile_error!(format!("`Parsable` derive could not infer type {}", #type_infer))},
        syn::Type::Never(..) => quote! { 
            compile_error!("`Parsable` cannot be derived for Never type")},
        syn::Type::Ptr(type_ptr) => quote! { 
            compile_error!(format!("`Parsable` cannot be derived for pointer type {}", #type_ptr))},
        syn::Type::Reference(type_reference) => quote! { 
            compile_error!(format!("`Parsable` cannot be derived for type reference {}", #type_reference))},
        syn::Type::Slice(type_slice) => quote! { 
            compile_error!(format!("`Parsable` cannot be derived for slice type {} (consider using `Repeat`)", #type_slice))},
        syn::Type::TraitObject(type_trait_object) => quote! { 
            compile_error!(format!("`Parsable` cannot be derived for trait object {}", #type_trait_object))},
        syn::Type::Verbatim(token_stream) => quote! {
            compile_error!(format!("`Parsable` derive could not infer type from {}", #token_stream))},
        ty => quote! { 
            compile_error!(format!("`Parsable` cannot be derived for {}", #ty))},
    }
}

fn impl_block(type_ident: &syn::Ident, inner_parse: TokenStream2) -> TokenStream2 {
    let type_name = &type_ident.to_string();
    quote! {
        impl<'a> leben_parsable::Parsable<'a> for #type_ident {
            fn parse(stream: &mut leben_parsable::ScopedStream<'a>) -> std::option::Option<Self> {
                #![allow(unexpected_cfgs)]
                #[cfg(leben_parsable_derive_debug)] {
                    println!("DEBUG >>>>>>>>>>> {}", #type_name);
                }
                let res = stream.scope(|stream| {
                    #inner_parse
                });
                #[cfg(leben_parsable_derive_debug)] {
                    println!("DEBUG <<<<<<<<<<< {}\n{:?}", #type_name, &res);
                }
                res
            }
        }
    }
}

fn struct_derive(type_ident: syn::Ident, data_struct: syn::DataStruct) -> TokenStream2 {
    match data_struct.fields {
        syn::Fields::Named(fields) => {
            let fields: Vec<_> = fields.named.iter().map(|field| {
                let name = &field.ident;
                let ty = &field.ty;
                let is_unit_type = is_unit_type(ty);

                if let Some(literal) = get_literal_attribute(&field.attrs) {
                    if is_unit_type {
                        quote! { #name: leben_parsable::parse_literal(stream, #literal)? }
                    } else {
                        quote! { #name: compile_error!("Unexpected `literal` attribute") }
                    }
                } else {
                    if is_unit_type {
                        quote! { #name: compile_error!("Expected `literal` attribute")  }
                    } else {
                        let member = parse_item(ty);
                        quote! { #name: #member }
                    }
                }
            }).collect();

            impl_block(
                &type_ident,
                quote! {
                    std::option::Option::Some(Self {
                        #( #fields ),*
                    })
                }
            ).into()
        },
        
        syn::Fields::Unnamed(fields) => {
            let fields: Vec<_> = fields.unnamed.iter().map(|field| {
                let ty = &field.ty;
                let is_unit_type = is_unit_type(ty);

                if let Some(literal) = get_literal_attribute(&field.attrs) {
                    if is_unit_type {
                        quote! { leben_parsable::parse_literal(stream, #literal)? }
                    } else {
                        quote! { compile_error!("Unexpected `literal` attribute") }
                    }
                } else {
                    if is_unit_type {
                        quote! { compile_error!("Expected `literal` attribute") }
                    } else {
                        let member = parse_item(ty);
                        quote! { #member }
                    }
                }
            }).collect();

            impl_block(
                &type_ident,
                quote! {
                    std::option::Option::Some(Self (
                        #( #fields ),*
                    ))
                }
            ).into()
        },
        
        syn::Fields::Unit => quote! { 
            compile_error!("`Parsable` cannot be derived for unit structs") 
        }.into(),
    }
}

fn enum_derive(enum_name: syn::Ident, data_enum: syn::DataEnum) -> TokenStream2 {
    let variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(fields) => {
                let fields: Vec<_> = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    let ty = &field.ty;
                    let is_unit_type = is_unit_type(ty);

                    if let Some(literal) = get_literal_attribute(&field.attrs) {
                        if is_unit_type {
                            quote! { #name: leben_parsable::parse_literal(stream, #literal)? }
                        } else {
                            quote! { #name: compile_error!("Unexpected `literal` attribute") }
                        }
                    } else {
                        if is_unit_type {
                            quote! { #name: compile_error!("Expected `literal` attribute")  }
                        } else {
                            let member = parse_item(ty);
                            quote! { #name: #member }
                        }
                    }
                }).collect();

                quote! {
                    std::option::Option::Some(Self::#variant_name {
                        #( #fields ),*
                    })
                }
            },
            syn::Fields::Unnamed(fields) => {
                let fields: Vec<_> = fields.unnamed.iter().map(|field| {
                    let ty = &field.ty;
                    let is_unit_type = is_unit_type(ty);

                    if let Some(literal) = get_literal_attribute(&field.attrs) {
                        if is_unit_type {
                            quote! { leben_parsable::parse_literal(stream, #literal)? }
                        } else {
                            quote! { compile_error!("Unexpected `literal` attribute") }
                        }
                    } else {
                        if is_unit_type {
                            quote! { compile_error!("Expected `literal` attribute") }
                        } else {
                            let member = parse_item(ty);
                            quote! { #member }
                        }
                    }
                }).collect();

                quote! {
                    std::option::Option::Some(Self::#variant_name(
                        #( #fields ),*
                    ))
                }
            },
            syn::Fields::Unit => quote! { 
                compile_error!("`Parsable` cannot be derived for unit enum variants") 
            }.into(),
        }
    });

    impl_block(
        &enum_name, 
        quote! {
            std::option::Option::None
                #( .or_else(|| #variants) )*
        }
    ).into()
}

fn get_literal_attribute(attributes: &Vec<syn::Attribute>) -> Option<syn::LitByteStr> {
    attributes.iter().find_map(|attr| {
        let name_value = if let syn::Meta::NameValue(name_value) = &attr.meta
            { name_value } else { return None; };
        if !name_value.path.is_ident("literal") { return None; }
        
        let lit_expr = if let syn::Expr::Lit(lit_expr) = &name_value.value
            { lit_expr } else { return None; };
        
        if let syn::Lit::ByteStr(lit_str) = &lit_expr.lit {
            Some(lit_str.clone())
        } else {
            None
        }
    })
}

fn is_unit_type(ty: &syn::Type) -> bool {
    if let syn::Type::Tuple(tuple_type) = ty {
        tuple_type.elems.empty_or_trailing()
    } else {
        false
    }
}
