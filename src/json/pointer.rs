#![allow(dead_code)]
use crate::json::utils::{escape, is_escaped, unescape};
use serde_json::Value;
use std::collections::VecDeque;
use std::fmt::Display;

/// A struct representing a JSON Pointer (RFC 6901).
///
/// This internally stores the pointer as an escaped string (e.g., "/a/~1b").
#[derive(Debug, Clone)]
pub struct Pointer(String);

impl Pointer {
    pub fn new(location: &str) -> Self {
        let mut location = location.to_string();
        if !is_escaped(&location) {
            location = escape(&location);
        }

        Self(location)
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
        unescape(tip)
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
        self.0 = String::new();
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
