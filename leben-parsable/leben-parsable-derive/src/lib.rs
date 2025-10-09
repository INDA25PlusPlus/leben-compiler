use proc_macro::TokenStream;
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
        syn::Data::Struct(data_struct) => struct_derive(input.ident, data_struct),

        syn::Data::Enum(data_enum) => enum_derive(input.ident, data_enum),
        
        syn::Data::Union(..) => quote! { 
            compile_error!("`Parsable` cannot be derived for unions") 
        }.into(),
    }
}

fn struct_derive(name: syn::Ident, data_struct: syn::DataStruct) -> TokenStream {
    match data_struct.fields {
        syn::Fields::Named(fields_named) => named_struct_derive(name, fields_named),
        
        syn::Fields::Unnamed(..) => quote! {
            compile_error!("`Parsable` cannot be derived for tuple structs") 
        }.into(),
        
        syn::Fields::Unit => quote! { 
            compile_error!("`Parsable` cannot be derived for unit structs") 
        }.into(),
    }
}

fn named_struct_derive(struct_name: syn::Ident, fields: syn::FieldsNamed) -> TokenStream {
    let fields: Vec<_> = fields.named.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        let literal = get_literal_attribute(&field.attrs);
        (name, ty, literal)
    }).collect();

    for (_, ty, literal) in &fields {
        let is_unit_type = is_unit_type(ty);
        match literal {
            Some(..) => if !is_unit_type {
                return quote! {
                    compile_error!("Unexpected `literal` attribute") 
                }.into();
            },
            None => if is_unit_type {
                return quote! {
                    compile_error!("Expected `literal` attribute") 
                }.into();
            },
        }
    }

    let fields = fields.iter().map(|(name, ty, literal)|
        if let Some(literal) = literal {
            quote! {
                #name: leben_parsable::parse_literal(stream, #literal)?
            }
        } else {
            quote! {
                #name: <#ty as leben_parsable::Parsable<'_>>::parse(stream)?
            }
        }
    );
    
    quote! {
        impl<'a> leben_parsable::Parsable<'a> for #struct_name {
            fn parse(stream: &mut leben_parsable::ScopedStream<'a>) -> Option<Self> {
                stream.scope(|stream| {
                    Some(Self {
                        #( #fields ),*
                    })
                })
            }
        }
    }.into()
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

fn enum_derive(name: syn::Ident, data_enum: syn::DataEnum) -> TokenStream {
    todo!()
}
