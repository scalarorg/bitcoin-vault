use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Lit};

pub fn impl_env_load(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Handle fields depending on their type
    let fields = match input.data {
        syn::Data::Struct(ref data) => &data.fields,
        _ => panic!("EnvLoad can only be used with structs"),
    };

    let field_initializers = match fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap(); // Safe to unwrap for Named fields
                let env_key = field
                    .attrs
                    .iter()
                    .find_map(|attr| {
                        let mut env_key = String::new();
                        if attr.path().is_ident("env_var") {
                            attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("key") {
                                    let lit = meta.value()?.parse::<Lit>()?;
                                    if let Lit::Str(lit) = lit {
                                        env_key = lit.value();
                                    }
                                }
                                Ok(())
                            })
                            .unwrap_or_else(|err| {
                                panic!("Failed to parse env_var attribute: {:?}", err)
                            });
                        }

                        match env_key.len() {
                            0 => None,
                            _ => Some(env_key),
                        }
                    })
                    .unwrap_or_else(|| field_name.to_string().to_uppercase());

                let field_type = &field.ty;
                if field_type == &syn::parse_quote!(u8) {
                    quote! {
                        #field_name: std::env::var(#env_key)
                            .map(|v| v.parse().unwrap_or_else(|_| default.#field_name))
                            .unwrap_or_else(|_| default.#field_name)
                    }
                } else if field_type == &syn::parse_quote!(Vec<String>) {
                    quote! {
                        #field_name: std::env::var(#env_key)
                        .map(|v| v.split(',').map(|s| s.to_string()).collect())
                        .unwrap_or_else(|_| default.#field_name.clone())
                    }
                } else {
                    quote! {
                        #field_name: std::env::var(#env_key)
                            .unwrap_or_else(|_| default.#field_name.clone())
                    }
                }
            })
            .collect::<Vec<_>>(),
        Fields::Unnamed(_) | Fields::Unit => {
            panic!("EnvLoad only supports structs with named fields")
        }
    };
    let expanded = quote! {
        impl #struct_name {
            pub fn load_from_env() -> Self {
                let default = Self::default();
                Self {
                    #(#field_initializers),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
