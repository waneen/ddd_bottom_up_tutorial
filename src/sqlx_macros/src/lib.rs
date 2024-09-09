extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(SqlxType)]
pub fn sqlx_type_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_sqlx_type(&input)
}

fn impl_sqlx_type(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let inner_type = extract_inner_type(ast);
    let gen = quote! {
        impl sqlx::Type<sqlx::Postgres> for #name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <#inner_type as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SqlxEncode)]
pub fn sqlx_encode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_sqlx_encode(&input)
}

fn impl_sqlx_encode(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let inner_type = extract_inner_type(ast);
    let gen = quote! {
        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for #name {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
                <#inner_type as sqlx::Encode<sqlx::Postgres>>::encode(self.0.clone(), buf)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SqlxDecode)]
pub fn sqlx_decode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_sqlx_decode(&input)
}

fn impl_sqlx_decode(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let inner_type = extract_inner_type(ast);
    let gen = quote! {
        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for #name {
            fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
                <#inner_type as sqlx::Decode<sqlx::Postgres>>::decode(value).map(Self)
            }
        }
    };
    gen.into()
}

fn extract_inner_type(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    if let Data::Struct(ref data_struct) = ast.data {
        if let Fields::Unnamed(ref fields_unnamed) = data_struct.fields {
            if fields_unnamed.unnamed.len() == 1 {
                let field = &fields_unnamed.unnamed[0];
                let ty = &field.ty;
                return quote! { #ty };
            }
        }
    }
    panic!("Expected a tuple struct with a single field");
}
