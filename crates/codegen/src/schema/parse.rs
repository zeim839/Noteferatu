use super::tree::*;
use super::attributes::*;

pub fn parse(input: &mut syn::DeriveInput) -> syn::Result<Tree> {
    for param in &mut input.generics.params {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(syn::parse_quote!(renfield::tools::Schema));
        }
    }
    Ok(Tree {
        name: &input.ident,
        vis: &input.vis,
        generics: &input.generics,
        data: parse_data(&input.data)?,
        attributes: parse_container_attributes(&input.attrs)?,
    })
}

fn parse_data(input: &syn::Data) -> syn::Result<DataNode> {
    match input {
        syn::Data::Struct(data) => Ok(DataNode::Struct(parse_struct(data)?)),
        syn::Data::Enum(data) => Ok(DataNode::Enum(parse_enum(data)?)),
        syn::Data::Union(data) => Err(syn::Error::new_spanned(
            data.union_token,
            "schema does not support derive for unions",
        )),
    }
}

fn parse_struct(input: &syn::DataStruct) -> syn::Result<StructNode> {
    match &input.fields {
        syn::Fields::Named(fields) => Ok(StructNode::Named(parse_named_fields(fields)?)),
        syn::Fields::Unnamed(fields) => Ok(StructNode::Unnamed(parse_unnamed_fields(fields)?)),
        syn::Fields::Unit => Ok(StructNode::Unit),
    }
}

fn parse_enum(input: &syn::DataEnum) -> syn::Result<EnumNode> {
    let mut variants = Vec::new();
    let mut errors = Vec::new();

    for variant in &input.variants {
        let attributes = parse_variant_attributes(&variant.attrs);
        if let Err(error) = attributes {
            errors.push(error);
            continue;
        }

        let attributes = attributes.unwrap();
        if attributes.skip {
            continue;
        }

        match &variant.fields {
            syn::Fields::Named(fields) => match parse_named_fields(fields) {
                Ok(fields) => variants.push((&variant.ident, StructNode::Named(fields), attributes)),
                Err(err) => errors.push(err),
            },
            syn::Fields::Unnamed(fields) => match parse_unnamed_fields(fields) {
                Ok(fields) => variants.push((&variant.ident, StructNode::Unnamed(fields), attributes)),
                Err(err) => errors.push(err),
            }
            syn::Fields::Unit => variants.push((&variant.ident, StructNode::Unit, attributes)),
        }
    }

    if !errors.is_empty() {
        let first = errors.remove(0);
        let err = errors.into_iter().fold(first, |mut acc, err| {
            acc.combine(err);
            acc
        });
        return Err(err);
    }

    Ok(EnumNode { variants })
}

fn parse_named_fields(input: &syn::FieldsNamed) -> syn::Result<NamedFieldsNode> {
    let mut fields = Vec::new();
    let mut errors = Vec::new();

    for field in &input.named {
        let attributes = parse_field_attributes(&field.attrs);
        if let Err(error) = attributes {
            errors.push(error);
            continue;
        }

        let attributes = attributes.unwrap();
        if attributes.skip {
            continue;
        }

        fields.push((field.ident.as_ref().unwrap(), &field.ty, attributes));
    }

    // Combine field errors into a single error.
    if !errors.is_empty() {
        let first = errors.remove(0);
        let err = errors.into_iter().fold(first, |mut acc, err| {
            acc.combine(err);
            acc
        });
        return Err(err);
    }

    Ok(NamedFieldsNode { fields })
}

fn parse_unnamed_fields(input: &syn::FieldsUnnamed) -> syn::Result<UnnamedFieldsNode> {
    let mut prefix_items = Vec::new();
    for field in &input.unnamed {
        prefix_items.push(&field.ty);
    }
    Ok(UnnamedFieldsNode{ prefix_items })
}
