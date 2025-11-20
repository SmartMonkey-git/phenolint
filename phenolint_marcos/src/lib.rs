mod doc_string;
mod utils;

use crate::doc_string::{check_rule_docs_format, extract_doc_string};
use crate::utils::{RuleArgs, extract_rule_id};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident as Ident2, Span as Span2};
use quote::quote;
use syn::{Item, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn register_rule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    let args = parse_macro_input!(attr as RuleArgs);
    let doc_string = extract_doc_string(&input);

    eprintln!("{:?}", args.id);
    eprintln!("{:?}", args.targets);
    check_rule_docs_format(&doc_string, &args.id);
    let struct_name = match &input {
        Item::Struct(item_struct) => &item_struct.ident,
        _ => panic!("register_rule can only be applied to structs"),
    };

    let upper_snake_case_struct_name = struct_name.to_string().to_case(Case::Snake).to_uppercase();
    let upper_snake_case_struct = Ident2::new(&upper_snake_case_struct_name, Span2::call_site());
    let rule_id = args.id.clone();
    let targets = &args.targets;

    let expanded = quote! {
        #input

        impl LintRule for #struct_name {
            fn rule_id(&self) -> &str { #rule_id }

            fn as_any(&self) -> &dyn Any { self }

            fn as_any_mut(&mut self) -> &mut dyn Any { self }

            fn supply_node_any(&mut self, node: &dyn Any, pointer: &Pointer) {

                #(
                    if let Some(typed_node) = node.downcast_ref::<#targets>() {
                        <Self as SupplyRule<#targets>>::supply_rule(self, pointer, typed_node);
                        return;
                    }
                )*


            }
        }

        static #upper_snake_case_struct: OnceLock<Arc<Result<Box<dyn LintRule>, FromContextError>>> = OnceLock::new();

        inventory::submit! {
            LintingPolicy {
                rule_id: #rule_id,
                factory: |context: &LinterContext| {
                    Arc::clone(#upper_snake_case_struct.get_or_init(|| {
                        Arc::new(#struct_name::from_context(context))
                    }))
                },
            }
        }

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
    };

    TokenStream::from(expanded)
}
