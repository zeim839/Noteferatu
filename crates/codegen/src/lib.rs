use proc_macro::TokenStream;

mod schema;
mod tool;

#[proc_macro_derive(schema, attributes(schema))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);
    match schema::parse(&mut input) {
        Ok(tree) => tree.generate().into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_derive(tool, attributes(tool))]
pub fn derive_tool(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match tool::generate(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
