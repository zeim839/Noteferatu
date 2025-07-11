/// Ensures that an object (i.e. drive or profile) is appropriately
/// named. Must be between 1-20 chars, start with a letter, and be
/// alphanumeric.
pub fn is_valid_name(name: &str) -> anyhow::Result<()> {
    if name.len() < 1 || name.len() > 20 {
        return Err(anyhow::anyhow!("bad name: must be between [1,20] chars"));
    }
    name.char_indices()
        .all(|(i, c)| if i == 0 {c.is_alphabetic()} else {c.is_alphanumeric()})
        .then(|| ())
        .ok_or(anyhow::anyhow!("bad name: use pattern [a-zA-Z][a-zA-Z0-9]"))
}
