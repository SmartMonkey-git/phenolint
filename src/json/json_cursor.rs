#![allow(dead_code)]
use crate::json::pointer::Pointer;
use crate::json::utils::escape;
use serde_json::Value;
use std::collections::VecDeque;

/// A navigational cursor for traversing and manipulating a JSON value tree.
///
/// `JsonCursor` wraps a [`serde_json::Value`] and maintains a [`Pointer`]
/// (similar to a JSON Pointer as defined in [RFC 6901](https://datatracker.ietf.org/doc/html/rfc6901))
/// that tracks the cursor’s current position within the JSON structure.
///
/// This is useful for iterative traversal, targeted lookups, or maintaining state
/// as you move around in a nested JSON document.
#[derive(Debug, Clone)]
pub(crate) struct JsonCursor {
    value: Value,
    pointer: Pointer,
    anchor: Option<Pointer>,
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
            pointer: Pointer::new(""),
            anchor: None,
        }
    }

    /// Moves the cursor directly to a new location represented by a [`Pointer`].
    ///
    /// This operation replaces the current pointer entirely.
    ///
    /// # Arguments
    /// * `ptr` - The new pointer.
    pub fn point_to(&mut self, ptr: &Pointer) -> &mut Self {
        self.pointer = ptr.clone();
        self
    }

    /// Searches for the first occurrence of a key within the JSON tree
    /// and sets the pointer to that location
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
                Value::Array(array) => (0..array.len() + 1)
                    .map(|index| index.to_string())
                    .collect(),
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

    /// Checks whether the cursor is currently pointing to a valid position
    /// in the JSON tree.
    ///
    /// # Returns
    /// `true` if the current pointer resolves to an existing value, `false` otherwise.
    pub fn is_valid_position(&self) -> bool {
        self.current_value().is_some()
    }

    /// Sets an anchor at the cursor's current position.
    ///
    /// The anchor can be used to mark a specific location in the JSON tree
    /// and return to it later using [`goto_anchor`](Self::goto_anchor).
    ///
    /// # Returns
    /// A mutable reference to the cursor (for chaining).
    pub fn set_anchor(&mut self) -> &mut Self {
        self.anchor = Some(self.pointer.clone());
        self
    }

    /// Clears the current anchor, if any.
    ///
    /// # Returns
    /// A mutable reference to the cursor (for chaining).
    pub fn clear_anchor(&mut self) -> &mut Self {
        self.anchor = None;
        self
    }

    /// Sets an anchor at a specific pointer location.
    ///
    /// # Arguments
    /// * `anchor` - A string representation of the pointer path to anchor at.
    ///
    /// # Returns
    /// A mutable reference to the cursor (for chaining).
    pub fn set_anchor_at(&mut self, anchor: &str) -> &mut Self {
        self.anchor = Some(Pointer::new(anchor));
        self
    }

    /// Moves the cursor to the previously set anchor position.
    ///
    /// If an anchor was set using [`set_anchor`](Self::set_anchor) or
    /// [`set_anchor_at`](Self::set_anchor_at), this method moves the cursor
    /// to that location and clears the anchor.
    ///
    /// # Returns
    /// A mutable reference to the cursor (for chaining).
    ///
    /// # Note
    /// If no anchor was set, the cursor position remains unchanged.
    pub fn goto_anchor(&mut self) -> &mut Self {
        match self.anchor.take() {
            None => self,
            Some(anchor) => {
                self.pointer = anchor;
                self
            }
        }
    }

    /// Returns a reference to the cursor's internal pointer.
    ///
    /// # Returns
    /// A reference to the [`Pointer`] representing the cursor's current position.
    pub fn pointer(&self) -> &Pointer {
        &self.pointer
    }

    /// Finds all locations where a predicate returns true.
    ///
    /// # Arguments
    /// * `predicate` - A function that takes a reference to a `Value` and returns `bool`.
    ///
    /// # Returns
    /// A vector of [`Pointer`]s pointing to all matching values.
    pub fn filter<F>(&self, predicate: F) -> Vec<Pointer>
    where
        F: Fn(&Value) -> bool,
    {
        self.iter_with_paths()
            .filter(|(_, v)| predicate(v))
            .map(|(ptr, _)| ptr)
            .collect()
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
        let new_ptr = Pointer::new("/user/name");
        cursor.point_to(&new_ptr);
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
        cursor.point_to(&Pointer::new("/nonexistent/path"));
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
