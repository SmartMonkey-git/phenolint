use proc_macro::TokenStream;
use regex::Regex;
use syn::parse::Parser;

use syn::{
    Ident, Lit, LitStr, Path, Result, Token, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

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

pub(crate) struct RuleArgs {
    pub id: String,
    pub targets: Vec<Path>,
}

impl Parse for RuleArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let rule_regex = Regex::new(RULE_FORMAT).unwrap();

        let mut id = None;
        let mut targets = None;

        // Loop through arguments separated by commas
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?; // Parse the '=' sign

            match key.to_string().as_str() {
                "id" => {
                    let s: LitStr = input.parse()?;

                    if rule_regex.is_match(&s.value()) {
                        id = Some(s.value());
                    } else {
                        let panic_message = "Invalid rule ID format. Rule needs to be of format ^[A-Z]{1,5}[0-9]{3}$";
                        panic!("{panic_message}");
                    };
                }
                // Handling the user's "tagets" (or "targets")
                "targets" | "tagets" => {
                    let content;
                    bracketed!(content in input);
                    let list: Punctuated<Path, Token![,]> =
                        content.parse_terminated(Path::parse, Token![,])?;
                    targets = Some(list.into_iter().collect());
                }
                _ => return Err(syn::Error::new(key.span(), "Unknown argument")),
            }

            // Check if there is another argument (comma separator)
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(RuleArgs {
            id: id.expect("Missing 'id' argument"),
            targets: targets.unwrap_or_default(),
        })
    }
}
