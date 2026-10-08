pub fn first_subword(mut s: String) -> String {
    if let Some((i, _)) = s
        .char_indices()
        .skip(1)
        .find(|(_, c)| c.is_uppercase() || *c == '_')
    {
        s.truncate(i);
    }
    s
}