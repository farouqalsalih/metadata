use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Data, Visibility};

#[proc_macro_derive(CS453Getters)]
pub fn derive_cs453getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => &named_fields.named,
            _ => return TokenStream::from(quote! { #input }),
        },
        _ => return TokenStream::from(quote! { #input }),
    };

    let public_fields: Vec<_> = fields.iter()
        .filter(|f| matches!(f.vis, Visibility::Public(_)))
        .collect();

    let getters = public_fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        let getter_name = syn::Ident::new(
            &format!("{}_cs453", field_name.as_ref().unwrap()),
            field_name.as_ref().unwrap().span()
        );
        quote! {
            pub fn #getter_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    });

    let expanded = quote! {
        #input

        impl #struct_name {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}
