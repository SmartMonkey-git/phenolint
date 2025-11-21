use proc_macro::TokenStream;
use regex::Regex;
use syn::parse::Parser;

use syn::Lit;

static RULE_FORMAT: &str = "^[A-Z]{1,5}[0-9]{3}$";

pub(crate) fn extract_rule_id(attr_tokens: &TokenStream) -> std::result::Result<String, String> {
    let mut rule_id = None;

    let attr_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("id") {
            let value: Lit = meta.value()?.parse()?;
            if let Lit::Str(lit_str) = value {
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

    attr_parser
        .parse(attr_tokens.clone())
        .map_err(|e| e.to_string())?;

    let rule_regex = Regex::new(RULE_FORMAT).unwrap();

    match rule_id {
        None => Err("Missing required `id = \"...\"` attribute argument".to_owned()),
        Some(rule_id) => {
            if rule_regex.is_match(&rule_id) {
                Ok(rule_id)
            } else {
                Err(
                    "Invalid rule ID format. Rule needs to be of format ^[A-Z]{1,5}[0-9]{3}$"
                        .to_owned(),
                )
            }
        }
    }
}
