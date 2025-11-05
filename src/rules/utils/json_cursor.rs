use serde_json::Value;
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Debug, Clone)]
struct Pointer(String);



fn escape(step: &str) -> String {
    let mut step = step.replace("⁓", "~0");
    step = step.replace("/", "~1");
    step
}

fn unescape(step: &str) -> String {
    let mut step = step.replace("~0", "⁓");
    step = step.replace("~1", "/");
    step
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
        let tip = self.0.split("/").last().unwrap_or_else(|| "");
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
    pub fn up(&mut self) -> &Self {
        let mut parts: Vec<&str> = self.0.split('/').collect();
        if !parts.is_empty() {
            parts.pop();
            self.0 = parts.join("");
            self
        } else {
            self
        }
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
    pub fn step<S: ToString>(&mut self, step: S) -> &Self {
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
    pub fn position(&self) -> String {
        let unescaped = unescape(self.0.as_str());
        unescaped
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
struct JsonCursor {
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
    /// * `leap` - The new pointer to jump to.
    pub fn jump(&mut self, leap: Pointer){
        self.pointer = leap;
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
    pub fn find_position(&mut self, target_key: &str) -> Option<Pointer> {
        let target = escape(target_key);
        for (_, pointer) in self.iter_with_paths(){
            if pointer.0.ends_with(&target){
                return Some(pointer);
            }
        }
        None
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
    pub fn find_positions(&mut self, target_key: &str) -> Vec<Pointer> {
        let target = escape(target_key);
        let mut result = Vec::new();
        for (_, pointer) in self.iter_with_paths(){
            if pointer.0.ends_with(&target){
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
    pub fn step<S: ToString>(&mut self, step: S) -> &mut Self {
        self.pointer.step(step);
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

    /// Returns a reference to the JSON value at the cursor's current position.
    ///
    /// # Returns
    /// * `Some(&Value)` if the pointer resolves to a valid location.
    /// * `None` if the pointer path does not exist.
    pub fn current_value(&self) -> Option<&Value> {
        self.value.pointer(self.pointer.position().as_str())
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
    pub fn iter_with_paths(&self) -> impl Iterator<Item = (&Value, Pointer)> {
        let mut queue = VecDeque::new();
        let current_value = self
            .value
            .pointer(self.pointer.position().as_str())
            .expect("");
        queue.push_back((current_value, self.pointer.clone()));

        std::iter::from_fn(move || {
            while let Some((value, pointer)) = queue.pop_front() {

                match value {
                    Value::Null => {
                    }
                    Value::Array(list) => {
                        for i in 0..list.len() {
                            let mut new_pointer = pointer.clone();
                            new_pointer.step(i);
                            let position = (&list[i], new_pointer);
                            queue.push_back(position.clone());
                        }
                    }
                    Value::Object(obj) => {
                        for (key, vale) in obj {
                            let mut new_pointer = pointer.clone();
                            new_pointer.step(key);
                            let position = (vale, new_pointer);

                            queue.push_back(position.clone());
                        }
                    }
                    _ => {}
                };

                return Some((value, pointer));
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
    #[rstest]
    fn test_iter_with_paths() {
        let test_json = json!({
            "string_field": "Hello, world!",
            "number_integer": 42,
            "number_float": 3.14159,
            "boolean_true": true,
            "boolean_false": false,
            "null_field": null,

            "array_of_numbers": [1, 2, 3, 4, 5],
            "array_of_strings": ["red", "green", "blue"],
            "array_of_objects": [
                { "id": 1, "name": "Alice" },
                { "id": 2, "name": "Bob" }
            ],

            "nested_object": {
                "level1": {
                    "level2": {
                        "message": "Deeply nested value",
                        "list": [true, false, null, "text"]
                    }
                }
            },

            // Optional fields: "optional_field" is present, "missing_optional_field" is omitted
            "optional_field": "present value",

            "enum_examples": {
                "simple_variant": { "type": "Unit" },
                "tuple_variant": { "type": "Tuple", "data": [10, "ten"] },
                "struct_variant": { "type": "Struct", "data": { "x": 1, "y": 2 } }
            },

            "map_example": {
                "one": 1,
                "two": 2,
                "three": 3
            },

            "mixed_array": [
                123,
                "text",
                { "nested": true },
                [1, 2, 3],
                null
            ]
        });

        JsonCursor::new(test_json)
            .iter_with_paths()
            .for_each(|(a, b)| println!("returned: {b} -> {a} "));
    }
}
