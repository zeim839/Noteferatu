use proc_macro::TokenStream;

// Tool module.

// Derives a function tool schema from a user-defined type.
#[proc_macro_derive(tool)]
pub fn derive_tool(_item: TokenStream) -> TokenStream {
    todo!();
}

// Bind custom attributes to tool schemas.
// E.g. tool name, tool description, field descriptions, etc.
#[proc_macro_attribute]
pub fn tool_info(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!();
}

// Model Context Protocol.

// Generates an MCP server handler.
#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!();
}
