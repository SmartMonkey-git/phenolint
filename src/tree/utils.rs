/// Escapes a string segment for use in a JSON Pointer.
///
/// Replaces "~" with "~0" and "/" with "~1".
pub(crate) fn escape(step: &str) -> String {
    if is_escaped(step) {
        step.to_string()
    } else {
        step.replace("~", "~0").replace("/", "~1")
    }
}

/// Unescapes a JSON Pointer segment.
///
/// Replaces "~1" with "/" and "~0" with "~".
pub(crate) fn unescape(step: &str) -> String {
    if is_escaped(step) {
        step.replace("~1", "/").replace("~0", "~")
    } else {
        step.to_string()
    }
}

pub(crate) fn is_escaped(step: &str) -> bool {
    let mut chars = step.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '~' {
            match chars.peek() {
                Some('0') | Some('1') => {
                    chars.next();
                }
                _ => return false,
            }
        }
    }
    true
}
