use syn::{Expr, Item, Lit, Meta};

pub(crate) fn extract_doc_string(input: &Item) -> String {
    match input {
        Item::Struct(item_struct) => item_struct
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .filter_map(|attr| match &attr.meta {
                Meta::NameValue(meta_name_value) => {
                    if let Expr::Lit(expr_lit) = &meta_name_value.value {
                        if let Lit::Str(doc_lit) = &expr_lit.lit {
                            return Some(doc_lit.value());
                        }
                    }
                    None
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n"),
        _ => panic!("register_rule needs a doc string"),
    }
}

pub(crate) fn check_rule_docs_format(docs: &str, rule_id: &str) {
    let doc_lines = docs.split('\n').collect::<Vec<_>>();

    if doc_lines.len() < 3 {
        panic!("Rule doc string needs at least 3 lines");
    }
    let expected_first_line = format!("### {rule_id}");
    let expected_headers = vec!["## What it does", "## Why is this bad?"];

    #[allow(clippy::collapsible_if)]
    if let Some(first_line) = doc_lines.first() {
        if !first_line.trim().starts_with(&expected_first_line) {
            panic!(
                "Rule docstring needs to start with '{expected_first_line}', but started witg {first_line}",
            );
        }
    }

    for header in expected_headers {
        if !doc_lines.iter().any(|l| l.trim().starts_with(header)) {
            panic!("Rule doc string needs to contain '{header}' section");
        }
    }
}
