#[derive(Default, Clone)]
pub struct Attributes {
    pub rename_all: Option<String>,
    pub rename: Option<String>,
    pub skip: bool,
    pub desc: Option<String>,
    pub optional: bool,
}

#[inline]
fn validate_rename(s: &syn::LitStr) -> syn::Result<()> {
    if syn::parse_str::<syn::Ident>(&s.value()).is_err() {
        return Err(syn::Error::new_spanned(s,
            "rename value must be valid variable name",
        ));
    }
    Ok(())
}

#[inline]
fn validate_rename_all(s: &syn::LitStr) -> syn::Result<()> {
    match s.value().as_str() {
        "UPPERCASE"
        | "lowercase"
        | "PascalCase"
        | "camelCase"
        | "snake_case"
        | "SCREAMING_SNAKE_CASE"
        | "kebab-case"
        | "SCREAMING-KEBAB-CASE" => Ok(()),
        _ => Err(syn::Error::new_spanned(s,
            format!("unknown rename rule {}", &s.value()),
        )),
    }
}

pub fn parse_field_attributes(attrs: &Vec<syn::Attribute>) -> syn::Result<Attributes> {
    let mut parsed = Attributes::default();
    let mut docstr: Option<String> = None;
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        match &mut docstr {
                            Some(s) => s.push_str(lit_str.value().as_str()),
                            None => { docstr = Some(lit_str.value()); },
                        }
                    }
                }
            }
            continue;
        }
        if !attr.path().is_ident("schema") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                parsed.skip = true;
                return Ok(());
            }
            if meta.path.is_ident("rename") {
                let s: syn::LitStr = meta.value()?.parse()?;
                validate_rename(&s)?;
                parsed.rename = Some(s.value());
                return Ok(());
            }
            if meta.path.is_ident("desc") {
                let s: syn::LitStr = meta.value()?.parse()?;
                parsed.desc = Some(s.value());
                return Ok(());
            }
            if meta.path.is_ident("optional") {
                if let Ok(value) = meta.value() {
                    let s: syn::LitBool = value.parse()?;
                    parsed.optional = s.value;
                    return Ok(());
                }
                parsed.optional = true;
                return Ok(());
            }
            Err(meta.error("unsupported attribute"))
        })?;
    }

    if parsed.desc.is_none() && docstr.is_some() {
        let docstr = docstr.unwrap();
        parsed.desc = Some(docstr.trim().to_string());
    }

    Ok(parsed)
}

pub fn parse_container_attributes(attrs: &Vec<syn::Attribute>) -> syn::Result<Attributes> {
    let mut parsed = Attributes::default();
    let mut docstr: Option<String> = None;
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        match &mut docstr {
                            Some(s) => s.push_str(lit_str.value().as_str()),
                            None => { docstr = Some(lit_str.value()); },
                        }
                    }
                }
            }
            continue;
        }
        if !attr.path().is_ident("schema") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                let s: syn::LitStr = meta.value()?.parse()?;
                validate_rename_all(&s)?;
                parsed.rename_all = Some(s.value());
                return Ok(());
            }
            if meta.path.is_ident("desc") {
                let s: syn::LitStr = meta.value()?.parse()?;
                parsed.desc = Some(s.value());
                return Ok(());
            }
            Err(meta.error("unsupported attribute"))
        })?;
    }

    if parsed.desc.is_none() && docstr.is_some() {
        let docstr = docstr.unwrap();
        parsed.desc = Some(docstr.trim().to_string());
    }

    Ok(parsed)
}

pub fn parse_variant_attributes(attrs: &Vec<syn::Attribute>) -> syn::Result<Attributes> {
    let mut parsed = Attributes::default();
    let mut docstr: Option<String> = None;
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        match &mut docstr {
                            Some(s) => s.push_str(lit_str.value().as_str()),
                            None => { docstr = Some(lit_str.value()); },
                        }
                    }
                }
            }
            continue;
        }
        if !attr.path().is_ident("schema") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                parsed.skip = true;
                return Ok(());
            }
            if meta.path.is_ident("rename") {
                let s: syn::LitStr = meta.value()?.parse()?;
                validate_rename(&s)?;
                parsed.rename = Some(s.value());
                return Ok(());
            }
            if meta.path.is_ident("desc") {
                let s: syn::LitStr = meta.value()?.parse()?;
                parsed.desc = Some(s.value());
                return Ok(());
            }
            Err(meta.error("unsupported attribute"))
        })?;
    }

    if parsed.desc.is_none() && docstr.is_some() {
        let docstr = docstr.unwrap();
        parsed.desc = Some(docstr.trim().to_string());
    }

    Ok(parsed)
}
