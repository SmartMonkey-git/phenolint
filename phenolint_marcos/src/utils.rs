use syn::Lit;

use proc_macro::TokenStream;
use syn::parse::Parser;

pub(crate) fn extract_rule_id(attr_tokens: TokenStream) -> Result<String, String> {
    let mut rule_id = None;

    // This parser closure captures the mutable `rule_id`
    let attr_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("id") {
            let value: Lit = meta.value()?.parse()?;
            if let Lit::Str(lit_str) = value {
                // Good practice: check for duplicates
                if rule_id.is_some() {
                    return Err(meta.error("duplicate `id` attribute argument"));
                }
                rule_id = Some(lit_str.value());
                Ok(())
            } else {
                Err(meta.error("`id` must be a string literal (e.g., `id = \"my-rule\"`)"))
            }
        } else {
            Err(meta.error("unsupported attribute argument, expected `id = \"...\"`"))
        }
    });

    // 1. THIS IS THE FIX: Actually *run* the parser on the input tokens
    // We map the `syn::Error` to a `String` to match your function signature.
    attr_parser.parse(attr_tokens).map_err(|e| e.to_string())?;

    // 2. ANOTHER FIX: Handle the case where `id` wasn't found
    // Instead of `unwrap()`, return an error if `rule_id` is still `None`.
    rule_id.ok_or_else(|| "Missing required `id = \"...\"` attribute argument".to_string())
}
