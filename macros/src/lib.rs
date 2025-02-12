use proc_macro::TokenStream;

mod env;

#[proc_macro_derive(EnvLoad, attributes(env_var))]
pub fn env_load_derive(input: TokenStream) -> TokenStream {
    env::impl_env_load(input)
}
