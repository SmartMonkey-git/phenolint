use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, Lit, parse_macro_input};

#[proc_macro_attribute]
pub fn lint_rule(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse as Item instead of DeriveInput to preserve the original tokens
    let input = parse_macro_input!(item as Item);

    // Extract the struct name
    let struct_name = match &input {
        Item::Struct(item_struct) => &item_struct.ident,
        _ => panic!("lint_rule can only be applied to structs"),
    };

    // Parse the attribute arguments
    let mut rule_id = None;
    let attr_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("id") {
            let value: Lit = meta.value()?.parse()?;
            if let Lit::Str(lit_str) = value {
                rule_id = Some(lit_str.value());
            }
            Ok(())
        } else {
            Err(meta.error("unsupported attribute argument"))
        }
    });

    parse_macro_input!(attr with attr_parser);

    let rule_id = rule_id.expect("lint_rule macro requires an `id` parameter");

    let expanded = quote! {
        #input

        // This is the new code the macro adds:
        impl LintRule for #struct_name {
            const RULE_ID: &'static str = #rule_id;
        }
        register_rule!(#struct_name);
    };

    TokenStream::from(expanded)
}
