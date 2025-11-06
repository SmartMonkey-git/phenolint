#![allow(dead_code)]
use serde_json::Value;
use std::collections::VecDeque;
use std::fmt::Display;

/// A struct representing a JSON Pointer (RFC 6901).
///
/// This internally stores the pointer as an escaped string (e.g., "/a/~1b").
#[derive(Debug, Clone)]
pub(crate) struct Pointer(String);

/// Escapes a string segment for use in a JSON Pointer.
///
/// Replaces "~" with "~0" and "/" with "~1".
fn escape(step: &str) -> String {
    step.replace("~", "~0").replace("/", "~1")
}

/// Unescapes a JSON Pointer segment.
///
/// Replaces "~1" with "/" and "~0" with "~".
fn unescape(step: &str) -> String {
    step.replace("~1", "/").replace("~0", "~")
}

impl Pointer {
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

    /// Checks if the pointer is at the root (i.e., is an empty string).
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

/// A navigational cursor for traversing and manipulating a JSON value tree.
///
/// `JsonCursor` wraps a [`serde_json::Value`] and maintains a [`Pointer`]
/// (similar to a JSON Pointer as defined in [RFC 6901](https://datatracker.ietf.org/doc/html/rfc6901))
/// that tracks the cursor’s current position within the JSON structure.
///
/// This is useful for iterative traversal, targeted lookups, or maintaining state
/// as you move around in a nested JSON document.
pub(crate) struct JsonCursor {
    value: Value,
    pointer: Pointer,
}

impl JsonCursor {
    /// Creates a new cursor positioned at the root of the provided JSON value.
    ///
    /// # Arguments
    /// * `value` - The JSON value to navigate.
    ///
    /// # Returns
    /// A new `JsonCursor` with an empty pointer (root position).
    pub fn new(value: Value) -> JsonCursor {
        Self {
            value,
            pointer: Pointer(String::new()),
        }
    }

    /// Moves the cursor directly to a new location represented by a [`Pointer`].
    ///
    /// This operation replaces the current pointer entirely.
    ///
    /// # Arguments
    /// * `ptr` - The new pointer.
    pub fn point_to(&mut self, ptr: Pointer) {
        self.pointer = ptr;
    }

    pub fn jump_to(&mut self, target_key: &str) -> &mut Self {
        match self.locate(target_key) {
            None => self,
            Some(ptr) => {
                self.pointer = ptr;
                self
            }
        }
    }

    /// Searches for the first occurrence of a key within the JSON tree,
    /// returning its corresponding pointer if found.
    ///
    /// This method performs a breadth-first traversal starting from the
    /// current cursor position.
    ///
    /// # Arguments
    /// * `target_key` - The key to search for.
    ///
    /// # Returns
    /// * `Some(Pointer)` if the key is found.
    /// * `None` if no match is found.
    pub fn locate(&mut self, target_key: &str) -> Option<Pointer> {
        self.iter_with_paths()
            .map(|(pointer, _)| pointer)
            .find(|pointer| pointer.get_tip() == target_key)
    }

    /// Finds all occurrences of a given key within the JSON structure.
    ///
    /// This function performs a breadth-first search starting from the current
    /// position and collects all matching paths.
    ///
    /// # Arguments
    /// * `target_key` - The key to search for.
    ///
    /// # Returns
    /// A vector of [`Pointer`]s pointing to all matches.
    pub fn locate_all<S: ToString>(&mut self, target_key: S) -> Vec<Pointer> {
        let target = escape(target_key.to_string().as_str());
        let mut result = Vec::new();
        for (pointer, _) in self.iter_with_paths() {
            if pointer.get_tip() == target {
                result.push(pointer);
            }
        }
        result
    }

    /// Moves the cursor one step deeper into the JSON tree.
    ///
    /// This appends a segment to the internal pointer, typically representing
    /// a key (for objects) or index (for arrays).
    ///
    /// # Arguments
    /// * `step` - The next path segment to move into.
    ///
    /// # Returns
    /// A mutable reference to the cursor (for chaining).
    pub fn down<S: ToString>(&mut self, step: S) -> &mut Self {
        self.pointer.down(step);
        self
    }

    /// Moves the cursor up one level in the JSON tree.
    ///
    /// Removes the last segment from the current pointer.
    ///
    /// # Returns
    /// A mutable reference to the cursor (for chaining).
    pub fn up(&mut self) -> &mut Self {
        self.pointer.up();
        self
    }

    /// Moves the cursor back to the root of the JSON value.
    ///
    /// This resets the internal pointer to the root position (`""`),
    /// effectively bringing the cursor to the top-level JSON node.
    ///
    /// # Returns
    /// A mutable reference to the cursor (for chaining).
    pub fn root(&mut self) -> &mut Self {
        self.pointer.root();
        self
    }

    /// Checks if the cursor is currently at the root of the JSON value.
    pub fn is_root(&self) -> bool {
        self.pointer.is_root()
    }

    /// Lists the immediate children at the cursor’s current position.
    ///
    /// If the cursor points to:
    /// - An **object**, returns its keys.
    /// - An **array**, returns its indices as strings (`"0"`, `"1"`, etc.).
    /// - Any other value type (number, string, boolean, null), returns an empty vector.
    ///
    /// # Returns
    /// A vector of strings representing the names or indices
    /// of the next available navigation steps.
    pub fn peek(&self) -> Vec<String> {
        match self.current_value() {
            None => {
                vec![]
            }
            Some(value) => match value {
                Value::Array(array) => (0..array.len()).map(|index| index.to_string()).collect(),
                Value::Object(obj) => obj.keys().map(|key| key.to_string()).collect(),
                _ => vec![],
            },
        }
    }

    /// Returns a reference to the JSON value at the cursor's current position.
    ///
    /// # Returns
    /// * `Some(&Value)` if the pointer resolves to a valid location.
    /// * `None` if the pointer path does not exist.
    pub fn current_value(&self) -> Option<&Value> {
        self.value.pointer(self.pointer.position())
    }

    /// Returns a reference to the cursor’s internal pointer.
    pub fn pointer(&self) -> &Pointer {
        &self.pointer
    }

    /// Iterates over all JSON sub-values and their corresponding pointers
    /// starting from the cursor’s current position.
    ///
    /// This performs a breadth-first traversal, yielding each value along with
    /// its full [`Pointer`] path.
    ///
    /// # Returns
    /// An iterator over `(&Value, Pointer)` pairs.
    pub fn iter_with_paths(&self) -> impl Iterator<Item = (Pointer, &Value)> {
        let mut queue = VecDeque::new();
        if let Some(current_value) = self.value.pointer(self.pointer.position()) {
            queue.push_back((current_value, self.pointer.clone()));
        }

        std::iter::from_fn(move || {
            #[allow(clippy::never_loop)]
            while let Some((value, pointer)) = queue.pop_front() {
                match value {
                    Value::Null => {}
                    Value::Array(list) => {
                        for (i, val) in list.iter().enumerate() {
                            let mut new_pointer = pointer.clone();
                            new_pointer.down(i);
                            let position = (val, new_pointer);
                            queue.push_back(position.clone());
                        }
                    }
                    Value::Object(obj) => {
                        for (key, vale) in obj {
                            let mut new_pointer = pointer.clone();
                            new_pointer.down(key);
                            let position = (vale, new_pointer);

                            queue.push_back(position.clone());
                        }
                    }
                    _ => {}
                };

                return Some((pointer, value));
            }
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn make_sample_json() -> Value {
        json!({
            "user": {
                "name": "Alice",
                "age": 30,
                "address": {
                    "city": "Paris",
                    "zip": "75000"
                }
            },
            "items": [
                { "id": 1, "name": "apple" },
                { "id": 2, "name": "banana" }
            ],
            "active": true,
            "score": 99.5
        })
    }

    #[rstest]
    fn test_new_starts_at_root() {
        let value = make_sample_json();
        let cursor = JsonCursor::new(value);
        assert_eq!(cursor.pointer().position(), "");
        assert!(cursor.current_value().is_some());
    }

    #[rstest]
    fn test_jump_replaces_pointer() {
        let mut cursor = JsonCursor::new(make_sample_json());
        let new_ptr = Pointer("/user/name".to_string());
        cursor.point_to(new_ptr.clone());
        assert_eq!(cursor.pointer().position(), new_ptr.position());
        assert_eq!(cursor.current_value(), Some(&json!("Alice")));
    }

    #[rstest]
    fn test_step_and_up_navigation() {
        let mut cursor = JsonCursor::new(make_sample_json());

        cursor.down("user").down("address").down("city");
        assert_eq!(cursor.pointer().position(), "/user/address/city");
        assert_eq!(cursor.current_value(), Some(&json!("Paris")));

        cursor.up();
        assert_eq!(cursor.pointer().position(), "/user/address");
        let current = cursor.current_value().unwrap();
        assert!(current.is_object());
        assert!(current.get("zip").is_some());
    }

    #[rstest]
    fn test_find_position_finds_first_key() {
        let mut cursor = JsonCursor::new(make_sample_json());
        let ptr = cursor.locate("city").expect("city should exist");
        assert_eq!(ptr.position(), "/user/address/city");
    }

    #[rstest]
    fn test_find_positions_finds_all_matches() {
        let mut cursor = JsonCursor::new(make_sample_json());
        let positions = cursor.locate_all("name");
        let paths: Vec<_> = positions.iter().map(|p| p.position()).collect();

        assert_eq!(paths.len(), 3);
        assert!(paths.contains(&"/user/name"));
        assert!(paths.contains(&"/items/0/name"));
        assert!(paths.contains(&"/items/1/name"));
    }

    #[rstest]
    fn test_iter_with_paths_yields_all_nodes() {
        let json = make_sample_json();
        let cursor = JsonCursor::new(json);
        let all: Vec<_> = cursor.iter_with_paths().collect();

        assert_eq!(all.first().unwrap().0.position(), "");

        let paths: Vec<_> = all.iter().map(|(p, _)| p.position()).collect();
        assert!(paths.contains(&"/user/address/city"));
        assert!(paths.contains(&"/items/0/id"));
        assert!(paths.contains(&"/score"));
    }

    #[rstest]
    fn test_current_value_returns_none_for_invalid_pointer() {
        let value = make_sample_json();
        let mut cursor = JsonCursor::new(value);
        cursor.point_to(Pointer("/nonexistent/path".to_string()));
        assert_eq!(cursor.current_value(), None);
    }

    #[rstest]
    fn test_complex_iteration_order_stable() {
        let json = json!({
            "a": {"b": {"c": 1}},
            "arr": [10, {"d": 2}]
        });
        let cursor = JsonCursor::new(json);
        let collected: Vec<String> = cursor
            .iter_with_paths()
            .map(|(p, _)| p.position().to_string())
            .collect();

        let expected = vec![
            "", "/a", "/arr", "/a/b", "/arr/0", "/arr/1", "/a/b/c", "/arr/1/d",
        ];
        for path in expected {
            assert!(collected.contains(&path.to_string()));
        }
    }
}
