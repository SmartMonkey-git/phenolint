use crate::error::InitError;
use crate::tree::pointer::Pointer;
use json_spanned_value::spanned::Value as SpannedValue;
use std::collections::HashMap;
use std::ops::Range;

pub(super) fn collect_json_spans(value: &str) -> Result<HashMap<Pointer, Range<usize>>, InitError> {
    let val = json_spanned_value::from_str(value)?;
    let mut out = HashMap::new();
    collect_spans_inner(&val, Pointer::at_root(), &mut out);
    Ok(out)
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
