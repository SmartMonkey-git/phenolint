use std::borrow::Cow;

/// Escapes a string segment for use in a JSON Pointer.
///
/// Replaces "~" with "~0" and "/" with "~1".
pub(super) fn escape(step: &str) -> Cow<'_, str> {
    step.replace("~", "~0").replace("/", "~1").into()
}

/// Unescapes a JSON Pointer segment.
///
/// Replaces "~1" with "/" and "~0" with "~".
pub(super) fn unescape(step: &str) -> Cow<'_, str> {
    if is_escaped(step) {
        step.replace("~1", "/").replace("~0", "~").into()
    } else {
        step.into()
    }
}

fn is_escaped(step: &str) -> bool {
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
