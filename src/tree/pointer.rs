use crate::tree::utils::{escape, unescape};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A struct representing a JSON Pointer (RFC 6901).
///
/// This internally stores the pointer as an escaped string (e.g., "/a/~1b").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pointer(String);

impl Pointer {
    pub fn new(location: &str) -> Self {
        let mut location = location.to_string();

        location = escape(&location);

        if !location.is_empty() && !location.starts_with("/") && !location.starts_with("~1") {
            location = format!("/{}", location);
        }

        Self(location)
    }

    pub fn at_root() -> Self {
        Self(String::new())
    }

    pub fn at_meta_data() -> Self {
        Self::new("metaData")
    }

    pub fn at_resources() -> Self {
        let mut mtd_ptr = Pointer::at_meta_data();
        mtd_ptr.down("resources");
        mtd_ptr
    }

    pub fn at_phenotypes() -> Self {
        Self::new("phenotypicFeatures")
    }

    pub fn at_subject() -> Self {
        Self::new("subject")
    }

    pub fn at_vital_status() -> Self {
        let mut ptr = Pointer::at_subject();
        ptr.down("vitalStatus");
        ptr
    }

    /// Returns the final segment (tip) of the pointer path.
    ///
    /// For example, if the pointer represents `"/user/name"`,
    /// this returns `"name"`.
    /// If the pointer is empty or at the root, it returns an empty string.
    ///
    /// # Returns
    /// A decoded string of the last path segment.
    pub fn get_tip(&self) -> String {
        let tip = self.0.split("/").last().unwrap_or("");
        tip.to_string()
    }

    /// Moves the pointer one level up the hierarchy.
    ///
    /// Removes the last path segment from the pointer string.
    /// If the pointer is already at the root, this is a no-op.
    ///
    /// # Returns
    /// A mutable reference to `self` (for chaining).
    ///
    /// # Example
    /// ```ignore
    /// let mut ptr = Pointer("/user/name".into());
    /// ptr.up();
    /// assert_eq!(ptr.position(), "/user");
    /// ```
    pub fn up(&mut self) -> &mut Self {
        if let Some(pos) = self.0.rfind('/') {
            self.0.truncate(pos);
        }
        self
    }

    /// Moves the pointer down one level by appending a new path segment.
    ///
    /// The provided segment is escaped as per JSON Pointer rules and appended
    /// to the existing path.
    ///
    /// # Arguments
    /// * `step` - The next path segment (object key or array index).
    ///
    /// # Returns
    /// A mutable reference to `self` (for chaining).
    ///
    /// # Example
    /// ```ignore
    /// let mut ptr = Pointer(String::new());
    /// ptr.step("user").step("name");
    /// assert_eq!(ptr.position(), "/user/name");
    /// ```
    pub fn down<S: ToString>(&mut self, step: S) -> &mut Self {
        let step = step.to_string();
        let step = escape(&step);
        self.0 = format!("{}/{}", self.0, step);

        self
    }

    /// Returns the full decoded string representation of the pointer.
    ///
    /// This is the current position as a JSON Pointer path (e.g. `"/user/name"`),
    /// with any escape sequences decoded.
    ///
    /// # Returns
    /// A string representing the pointer’s current position.
    pub fn position(&self) -> &str {
        self.0.as_str()
    }

    /// Resets the pointer to the root position (`""`).
    ///
    /// # Returns
    /// A mutable reference to `self` (for chaining).
    pub fn root(&mut self) -> &mut Self {
        self.0 = "".to_owned();
        self
    }

    /// Checks if the cursor is currently at the root of the JSON value.
    ///
    /// # Returns
    /// `true` if the cursor is at the root position, `false` otherwise.
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn segments(&self) -> impl Iterator<Item = String> + '_ {
        self.0.split('/').skip(1).map(unescape)
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    #[rstest]
    fn test_new_empty() {
        let ptr = Pointer::new("");
        assert_eq!(ptr.position(), "");
        assert!(ptr.is_root());
    }

    #[rstest]
    fn test_new_with_leading_slash() {
        let ptr = Pointer::new("/foo/bar");
        assert_eq!(ptr.position(), "/foo/bar");
    }

    #[rstest]
    fn test_new_without_leading_slash() {
        let ptr = Pointer::new("foo/bar");
        assert_eq!(ptr.position(), "/foo/bar");
    }

    #[rstest]
    fn test_new_escapes_special_chars() {
        let ptr = Pointer::new("/foo/a~b/c/d");
        // Should escape ~ to ~0 and / to ~1
        assert_eq!(ptr.position(), "~1foo~1a~0b~1c~1d");
    }

    #[rstest]
    fn test_new_with_slash_in_segment() {
        let ptr = Pointer::new("a/b");
        // The slash should be escaped
        assert!(ptr.position().contains("~1") || ptr.position() == "/a/b");
    }

    #[rstest]
    fn test_get_tip_simple() {
        let ptr = Pointer::new("/user/name");
        assert_eq!(ptr.get_tip(), "name");
    }

    #[rstest]
    fn test_get_tip_root() {
        let ptr = Pointer::new("");
        assert_eq!(ptr.get_tip(), "");
    }

    #[rstest]
    fn test_get_tip_single_segment() {
        let ptr = Pointer::new("/foo");
        assert_eq!(ptr.get_tip(), "foo");
    }

    #[rstest]
    fn test_get_tip_with_escaped_chars() {
        let ptr = Pointer::new("/user/na~0me");
        let tip = ptr.get_tip();
        assert_eq!(tip, "na~0me");
    }

    #[rstest]
    fn test_up_from_nested() {
        let mut ptr = Pointer::new("/user/name/first");
        ptr.up();
        assert_eq!(ptr.position(), "/user/name");
    }

    #[rstest]
    fn test_up_multiple_times() {
        let mut ptr = Pointer::new("/a/b/c/d");
        ptr.up();
        assert_eq!(ptr.position(), "/a/b/c");
        ptr.up();
        assert_eq!(ptr.position(), "/a/b");
        ptr.up();
        assert_eq!(ptr.position(), "/a");
        ptr.up();
        assert_eq!(ptr.position(), "");
    }

    #[rstest]
    fn test_up_at_root() {
        let mut ptr = Pointer::new("");
        ptr.up();
        assert_eq!(ptr.position(), "");
        assert!(ptr.is_root());
    }

    #[rstest]
    fn test_up_chaining() {
        let mut ptr = Pointer::new("/a/b/c");
        ptr.up().up();
        assert_eq!(ptr.position(), "/a");
    }

    #[rstest]
    fn test_down_simple() {
        let mut ptr = Pointer::new("");
        ptr.down("user");
        assert_eq!(ptr.position(), "/user");
    }

    #[rstest]
    fn test_down_multiple() {
        let mut ptr = Pointer::new("");
        ptr.down("user").down("name");
        assert_eq!(ptr.position(), "/user/name");
    }

    #[rstest]
    fn test_down_with_special_chars() {
        let mut ptr = Pointer::new("");
        ptr.down("a~b");
        // ~ should be escaped to ~0
        assert_eq!(ptr.position(), "/a~0b");
    }

    #[rstest]
    fn test_down_with_integer() {
        let mut ptr = Pointer::new("/array");
        ptr.down(0);
        assert_eq!(ptr.position(), "/array/0");
    }

    #[rstest]
    fn test_position() {
        let ptr = Pointer::new("/foo/bar");
        assert_eq!(ptr.position(), "/foo/bar");
    }

    #[rstest]
    fn test_root() {
        let mut ptr = Pointer::new("/user/name");
        ptr.root();
        assert_eq!(ptr.position(), "");
        assert!(ptr.is_root());
    }

    #[rstest]
    fn test_root_chaining() {
        let mut ptr = Pointer::new("/a/b/c");
        ptr.root().down("new");
        assert_eq!(ptr.position(), "/new");
    }

    #[rstest]
    fn test_is_root_true() {
        let ptr = Pointer::new("");
        assert!(ptr.is_root());
    }

    #[rstest]
    fn test_is_root_false() {
        let ptr = Pointer::new("/foo");
        assert!(!ptr.is_root());
    }

    #[rstest]
    fn test_segments_empty() {
        let ptr = Pointer::new("");
        let segments: Vec<String> = ptr.segments().collect();
        assert_eq!(segments, Vec::<String>::new());
    }

    #[rstest]
    fn test_segments_single() {
        let ptr = Pointer::new("/foo");
        let segments: Vec<String> = ptr.segments().collect();
        assert_eq!(segments, vec!["foo"]);
    }

    #[rstest]
    fn test_segments_multiple() {
        let ptr = Pointer::new("/foo/bar/baz");
        let segments: Vec<String> = ptr.segments().collect();
        assert_eq!(segments, vec!["foo", "bar", "baz"]);
    }

    #[rstest]
    fn test_segments_with_escaped_chars() {
        let ptr = Pointer::new("/foo/a~0b/c~1d");
        let segments: Vec<String> = ptr.segments().collect();
        // Segments should be unescaped
        assert_eq!(segments, vec!["foo", "a~b", "c/d"]);
    }

    #[rstest]
    fn test_display_trait() {
        let ptr = Pointer::new("/user/name");
        assert_eq!(format!("{}", ptr), "/user/name");
    }

    #[rstest]
    fn test_clone() {
        let ptr1 = Pointer::new("/foo/bar");
        let ptr2 = ptr1.clone();
        assert_eq!(ptr1, ptr2);
        assert_eq!(ptr1.position(), ptr2.position());
    }

    #[rstest]
    fn test_equality() {
        let ptr1 = Pointer::new("/foo/bar");
        let ptr2 = Pointer::new("/foo/bar");
        assert_eq!(ptr1, ptr2);
    }

    #[rstest]
    fn test_inequality() {
        let ptr1 = Pointer::new("/foo/bar");
        let ptr2 = Pointer::new("/foo/baz");
        assert_ne!(ptr1, ptr2);
    }

    #[rstest]
    fn test_complex_navigation() {
        let mut ptr = Pointer::new("");
        ptr.down("users")
            .down("john")
            .down("address")
            .down("street");
        assert_eq!(ptr.position(), "/users/john/address/street");
        assert_eq!(ptr.get_tip(), "street");

        ptr.up();
        assert_eq!(ptr.position(), "/users/john/address");
        assert_eq!(ptr.get_tip(), "address");

        ptr.down("city");
        assert_eq!(ptr.position(), "/users/john/address/city");
    }

    #[rstest]
    fn test_empty_segment() {
        let ptr = Pointer::new("//");
        // Should handle empty segments
        let segments: Vec<String> = ptr.segments().collect();
        assert_eq!(segments.len(), 2);
    }

    #[rstest]
    fn test_numeric_string_segment() {
        let mut ptr = Pointer::new("");
        ptr.down("123");
        assert_eq!(ptr.position(), "/123");
        assert_eq!(ptr.get_tip(), "123");
    }

    #[rstest]
    fn test_unicode_segments() {
        let mut ptr = Pointer::new("");
        ptr.down("ユーザー").down("名前");
        assert_eq!(ptr.get_tip(), "名前");
    }

    #[rstest]
    fn test_special_json_pointer_chars() {
        let mut ptr = Pointer::new("");
        ptr.down("~/test");

        assert!(ptr.position().contains("~0"));
        assert!(ptr.position().contains("~1"));
    }
}
