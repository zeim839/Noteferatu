use proc_macro2::TokenStream;
use super::attributes::*;

pub fn generate(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let attributes = parse_container_attributes(&input.attrs)?;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (vis, ident_name) = (input.vis, input.ident);

    let tool_name = attributes.rename.unwrap_or(ident_name.to_string());
    let desc = attributes.desc
        .map(|d| quote::quote!{ "description": #d, })
        .unwrap_or(quote::quote!{});

    let strict = attributes.strict
        .then(|| quote::quote!{ "strict": true, })
        .unwrap_or(quote::quote!{});

    Ok(quote::quote!{
        impl #impl_generics renfield::tools::Tool for #ident_name #ty_generics #where_clause {
            #vis fn as_anthropic_tool() -> serde_json::Value {
                serde_json::json!({
                    "name": #tool_name,
                    "input_schema": <#ident_name as renfield::tools::Schema>::schema(),
                    #desc
                })
            }

            #vis fn as_openai_tool() -> serde_json::Value {
                serde_json::json!({
                    "type": "function",
                    "name": #tool_name,
                    "parameters": <#ident_name as renfield::tools::Schema>::schema(),
                    #strict
                    #desc
                })
            }

            #vis fn as_gemini_tool() -> serde_json::Value {
                serde_json::json!({
                    "name": #tool_name,
                    "parameters": <#ident_name as renfield::tools::Schema>::schema(),
                    #desc
                })
            }

            #vis fn as_openrouter_tool() -> serde_json::Value {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": #tool_name,
                        "parameters": <#ident_name as renfield::tools::Schema>::schema(),
                        #desc
                    },
                })
            }

            #vis fn as_ollama_tool() -> serde_json::Value {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": #tool_name,
                        "parameters": <#ident_name as renfield::tools::Schema>::schema(),
                        #desc
                    },
                })
            }
        }
    })
}
