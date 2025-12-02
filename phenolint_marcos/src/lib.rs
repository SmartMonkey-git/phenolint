mod doc_string;
mod utils;

use crate::doc_string::{check_rule_docs_format, extract_doc_string};
use crate::utils::{extract_rule_id, generate_rule_report_assertion};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn register_rule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    let doc_string = extract_doc_string(&input);
    let rule_id = match extract_rule_id(&attr) {
        Ok(rule_id) => rule_id,
        Err(err) => panic!("{}", err),
    };

    check_rule_docs_format(&doc_string, &rule_id);
    let struct_name = match &input {
        Item::Struct(item_struct) => &item_struct.ident,
        _ => panic!("register_rule can only be applied to structs"),
    };

    let rule_report_assertion = generate_rule_report_assertion(&rule_id);

    let expanded = quote! {
        #input

        impl RuleMetaData for #struct_name {
            fn rule_id(&self) -> &str { #rule_id }
        }

        inventory::submit! {
            RuleRegistration {
                rule_id: #rule_id,
                factory: |context: &LinterContext| {
                    #struct_name::from_context(context)
                },
            }
        }

        const _: () = {
            fn _assert_report_exists() {
                let _ = #rule_report_assertion::RULE_ID;
            }
        };
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn register_patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let rule_id = match extract_rule_id(&attr) {
        Ok(rule_id) => rule_id,
        Err(err) => panic!("{}", err),
    };

    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #input
        impl RulePatch for #name {
            const RULE_ID: &'static str = #rule_id;
        }

        ::inventory::submit! {
            PatchRegistration {
                rule_id: #rule_id,
                factory: |context: &LinterContext| {
                    #name::from_context(context)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn register_report(attr: TokenStream, item: TokenStream) -> TokenStream {
    let rule_id = match extract_rule_id(&attr) {
        Ok(rule_id) => rule_id,
        Err(err) => panic!("{}", err),
    };

    let rule_report_assertion = generate_rule_report_assertion(&rule_id);

    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #input
        impl RuleReport for #name {
            const RULE_ID: &'static str = #rule_id;
        }

        ::inventory::submit! {
            ReportRegistration {
                rule_id: #rule_id,
                factory: |context: &LinterContext| {
                    #name::from_context(context)
                }
            }
        }

        pub(crate) struct #rule_report_assertion;
        impl #rule_report_assertion{
            pub(crate) const RULE_ID: &'static str = #rule_id;
        }

    };

    TokenStream::from(expanded)
}
