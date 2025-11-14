/*use json_spanned_value::spanned::Value as SpannedValue;
use phenolint::tree::pointer::Pointer;
use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;

#[test]
fn test_flex() {
    let test_pp = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("phenopacket.json");
    let pp = fs::read(test_pp).unwrap();
    let json = serde_json::to_string_pretty(&pp).unwrap();
    let val = json_spanned_value::from_slice(pp.as_slice()).unwrap();

    let a = collect_spans(&val);

    println!("{:?}", a);
}

pub fn collect_spans(value: &str) -> HashMap<Pointer, Range<usize>> {
    let val = json_spanned_value::from_slice(value.as_slice()).unwrap();
    let mut out = HashMap::new();
    collect_spans_inner(val, Pointer::at_root(), &mut out);
    out
}

fn collect_spans_inner(
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
            collect_spans_inner(val, new_ptr, out);
        }
        return;
    }

    // Array case
    if let Some(arr) = value.as_span_array() {
        for (idx, val) in arr.iter().enumerate() {
            let mut new_ptr = path.clone();
            new_ptr.down(idx.to_string());
            collect_spans_inner(val, new_ptr, out);
        }
    }

    // Primitive → nothing else to descend into
}
*/
