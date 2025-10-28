use super::attributes::Attributes;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;

pub struct Tree<'a> {
    pub name: &'a syn::Ident,
    pub vis: &'a syn::Visibility,
    pub generics: &'a syn::Generics,
    pub data: DataNode<'a>,
    pub attributes: Attributes,
}

impl<'a> Tree<'a> {
    pub fn generate(&self) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let (name, vis) = (self.name, self.vis);
        let inner = self.data.generate(&self.attributes);
        quote::quote! {
            impl #impl_generics renfield::tools::Schema for #name #ty_generics #where_clause {
                #vis fn schema() -> serde_json::Value {
                    #inner
                }
            }
        }
    }
}

pub enum DataNode<'a> {
    Struct(StructNode<'a>),
    Enum(EnumNode<'a>),
}

impl<'a> DataNode<'a> {
    pub fn generate(&self, global_attrs: &Attributes) -> TokenStream {
        match self {
            Self::Struct(data) => data.generate(global_attrs),
            Self::Enum(data) => data.generate(global_attrs),
        }
    }
}

pub enum StructNode<'a> {
    Named(NamedFieldsNode<'a>),
    Unnamed(UnnamedFieldsNode<'a>),
    Unit,
}

impl<'a> StructNode<'a> {
    pub fn generate(&self, global_attrs: &Attributes) -> TokenStream {
        match self {
            Self::Named(fields) => fields.generate(global_attrs),
            Self::Unnamed(fields) => fields.generate(global_attrs),
            Self::Unit => {
                let desc = global_attrs.desc.clone()
                    .map(|d| quote::quote!{ "description": #d, })
                    .unwrap_or_default();

                quote::quote!{
                    serde_json::json!({
                        "type": "object",
                        "properties": {},
                        "additionalProperties": false,
                        #desc
                    })
                }
            },
        }
    }
}

pub struct EnumNode<'a> {
    pub variants: Vec<(&'a syn::Ident, StructNode<'a>, Attributes)>,
}

impl<'a> EnumNode<'a> {
    pub fn generate(&self, global_attrs: &Attributes) -> TokenStream {
        let mut enum_items = Vec::new();
        for variant in &self.variants {
            match &variant.1 {
                StructNode::Named(fields) => {
                    let mut attrs = global_attrs.clone();
                    attrs.desc = variant.2.desc.clone();
                    enum_items.push(fields.generate(&attrs));
                },
                StructNode::Unnamed(fields) => {
                    let mut attrs = global_attrs.clone();
                    attrs.desc = variant.2.desc.clone();
                    enum_items.push(fields.generate(&attrs));
                },
                StructNode::Unit => {
                    let variant_name = variant.2.rename.clone()
                        .unwrap_or_else(|| {
                            global_attrs.rename_all.clone()
                                .map(|case| convert_case(&case, &variant.0.to_string()))
                                .unwrap_or(variant.0.to_string())
                        });

                    enum_items.push(quote::quote!{ #variant_name });
                },
            }
        }

        let desc = global_attrs.desc.clone()
            .map(|d| quote::quote!{ "description": #d, })
            .unwrap_or_default();

        quote::quote! {
            serde_json::json!({
                "enum": [ #( #enum_items ),* ],
                #desc
            })
        }
    }
}

pub struct NamedFieldsNode<'a> {
    pub fields: Vec<(&'a syn::Ident, &'a syn::Type, Attributes)>,
}

impl<'a> NamedFieldsNode<'a> {
    pub fn generate(&self, global_attrs: &Attributes) -> TokenStream {
        let mut properties = Vec::new();
        let mut required = Vec::new();
        for field in &self.fields {
            let ident_type = &field.1;
            let ident_name = field.2.rename.clone().unwrap_or_else(|| {
                global_attrs.rename_all.clone()
                    .map(|s| convert_case(&s, &field.0.to_string()))
                    .unwrap_or(field.0.to_string())
            });

            // Non-option fields are declared as required.
            if !is_option(ident_type) && !field.2.optional {
                required.push(quote::quote!{ #ident_name });
            }

            let desc = field.2.desc.clone()
                .map(|v| quote::quote!{ Some(#v) })
                .unwrap_or(quote::quote! { None });

            properties.push(quote::quote! {
                #ident_name: <#ident_type>::schema_with_meta(#desc)
            });
        }

        // Adds a `required: [...]` field to the JSON schema.
        let required = required.is_empty().then(|| quote::quote! {})
            .unwrap_or(quote::quote! {
                "required": [ #( #required),* ],
            });

        let desc = global_attrs.desc.clone()
            .map(|d| quote::quote!{ "description": #d, })
            .unwrap_or_default();

        quote::quote!{
            serde_json::json!({
                "type": "object",
                "properties": { #( #properties ),* },
                #desc
                #required
            })
        }
    }
}

pub struct UnnamedFieldsNode<'a> {
    pub prefix_items: Vec<&'a syn::Type>,
}

impl<'a> UnnamedFieldsNode<'a> {
    pub fn generate(&self, global_attrs: &Attributes) -> TokenStream {
        let desc = global_attrs.desc.clone()
            .map(|d| quote::quote!{ Some(#d) })
            .unwrap_or(quote::quote!{ None });

        match &self.prefix_items.len() {
            0 => quote::quote! { <()>::schema_with_meta(#desc) },
            1 => {
                let field_type = self.prefix_items[0];
                quote::quote!{ <#field_type>::schema_with_meta(#desc) }
            }
            _ => {
                let mut schemas = Vec::new();
                for item in &self.prefix_items {
                    let field_type = item;
                    schemas.push(quote::quote! {
                        <#field_type>::schema()
                    })
                }

                let desc = global_attrs.desc.clone()
                    .map(|d| quote::quote!{ "description": #d, })
                    .unwrap_or_default();

                quote::quote! {
                    serde_json::json!({
                        "type": "array",
                        "prefixItems": [
                            #( #schemas ),*
                        ],
                        #desc
                    })
                }
            }
        }
    }
}

#[inline]
fn is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

#[inline]
fn convert_case(case: &str, name: &str) -> String {
    match case {
        "UPPERCASE" => name.to_case(Case::UpperFlat),
        "lowercase" => name.to_case(Case::Lower),
        "PascalCase" => name.to_case(Case::Pascal),
        "camelCase" => name.to_case(Case::Camel),
        "snake_case" => name.to_case(Case::Snake),
        "SCREAMING_SNAKE_CASE" => name.to_case(Case::UpperSnake),
        "kebab-case" => name.to_case(Case::Kebab),
        "SCREAMING-KEBAB-CASE" => name.to_case(Case::UpperKebab),
        _ => String::new(),
    }
}
