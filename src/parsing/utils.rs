use crate::error::ParsingError;
use crate::tree::pointer::Pointer;
use json_spanned_value::spanned::Value as SpannedValue;
use saphyr::{LoadableYamlNode, MarkedYaml, YamlData};
use std::collections::HashMap;
use std::ops::Range;

pub(super) fn collect_json_spans(
    value: &str,
) -> Result<HashMap<Pointer, Range<usize>>, ParsingError> {
    let val = json_spanned_value::from_str(value)?;
    let mut out = HashMap::new();
    collect_json_spans_inner(&val, Pointer::at_root(), &mut out);
    Ok(out)
}

fn collect_json_spans_inner(
    value: &SpannedValue,
    path: Pointer,
    out: &mut HashMap<Pointer, Range<usize>>,
) {
    out.insert(path.clone(), value.start()..value.end());

    // Object case
    if let Some(obj) = value.as_span_object() {
        for (key, val) in obj.iter() {
            let mut new_ptr = path.clone();
            new_ptr.down(key);
            collect_json_spans_inner(val, new_ptr, out);
        }
        return;
    }

    // Array case
    if let Some(arr) = value.as_span_array() {
        for (idx, val) in arr.iter().enumerate() {
            let mut new_ptr = path.clone();
            new_ptr.down(idx.to_string());
            collect_json_spans_inner(val, new_ptr, out);
        }
    }
}

/// Collect all spans for a `Spanned<Value>` YAML structure
pub(crate) fn collect_yaml_spans(
    yaml_str: &str,
) -> Result<HashMap<Pointer, Range<usize>>, ParsingError> {
    let mut spans: HashMap<Pointer, Range<usize>> = HashMap::new();

    let yaml = MarkedYaml::load_from_str(yaml_str)?;
    for yaml_node in yaml {
        collect_yaml_spanns_inner(&yaml_node, Pointer::at_root(), &mut spans);
    }
    Ok(spans)
}

fn collect_yaml_spanns_inner(
    node: &MarkedYaml,
    path: Pointer,
    spans: &mut HashMap<Pointer, Range<usize>>,
) {
    match &node.data {
        YamlData::Mapping(map) => {
            for (key, value) in map {
                if let YamlData::Value(key_str) = &key.data {
                    let mut new_path = path.clone();
                    new_path.down(key_str.as_str().unwrap());
                    spans.insert(
                        new_path.clone(),
                        value.span.start.index()..value.span.end.index(),
                    );
                    collect_yaml_spanns_inner(value, new_path, spans);
                }
            }
        }
        YamlData::Sequence(seq) => {
            for (idx, item) in seq.iter().enumerate() {
                let mut new_path = path.clone();
                new_path.down(idx.to_string());
                spans.insert(
                    new_path.clone(),
                    item.span.start.index()..item.span.end.index() - 1,
                );
                collect_yaml_spanns_inner(item, new_path, spans);
            }
        }
        _ => {}
    }
}
