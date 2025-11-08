use crate::json::error::JsonEditError;
use crate::json::{JsonCursor, Pointer};
use serde_json::Value;
use std::ops::{Deref, DerefMut};

pub(crate) struct JsonEditor {
    cursor: JsonCursor,
}

impl JsonEditor {
    pub fn new(cursor: JsonCursor) -> Self {
        Self { cursor }
    }

    pub fn export(&self) -> Result<String, JsonEditError> {
        Ok(serde_json::to_string_pretty(self.json())?)
    }

    // 1. Get mutable reference to current value
    /// Returns a mutable reference to the JSON value at the cursor's current position.
    ///
    /// # Returns
    /// * `Some(&mut Value)` if the pointer resolves to a valid location.
    /// * `None` if the pointer path does not exist.
    fn current_value_mut(&mut self) -> Option<&mut Value> {
        let position = self.pointer().position().to_string();
        self.cursor.json_mut().pointer_mut(&position)
    }

    // 2. Replace/update value at current position
    /// Replaces the value at the cursor's current position with a new value.
    ///
    /// # Arguments
    /// * `new_value` - The value to insert at the current position.
    ///
    /// # Returns
    /// * `Ok(())` if the replacement was successful.
    /// * `Err(())` if the current pointer doesn't point to a valid location.
    pub fn set_value(&mut self, new_value: Value) -> Result<(), JsonEditError> {
        if let Some(target) = self.current_value_mut() {
            *target = new_value;
            Ok(())
        } else {
            Err(JsonEditError::InvalidPosition(self.pointer().clone()))
        }
    }

    /// Removes the value at the cursor's current position.
    ///
    /// For arrays, this removes the element and shifts subsequent elements.
    /// For objects, this removes the key-value pair.
    ///
    /// # Returns
    /// * `Some(Value)` containing the removed value if successful.
    /// * `None` if the position is invalid or at root.
    pub fn delete(&mut self) -> Option<Value> {
        if self.is_root() {
            return None;
        }

        self.up();
        let key = self.pointer().get_tip();
        let to_del = self.pointer().position().to_string();

        let parent = self.json_mut().pointer_mut(&to_del)?;

        match parent {
            Value::Object(map) => map.remove(&key),
            Value::Array(arr) => {
                if let Ok(idx) = key.parse::<usize>() {
                    if idx < arr.len() {
                        Some(arr.remove(idx))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    pub fn push(&mut self, value: Value, construct_path: bool) -> Result<&mut Self, JsonEditError> {
        let current_ptr = self.pointer().position().to_string();

        // Try to get the current location
        let current = self.json_mut().pointer_mut(&current_ptr);

        if current.is_none() && construct_path {
            // Path doesn't exist, construct it
            self.construct_path_to(&current_ptr)?;
        }

        let current = self
            .json_mut()
            .pointer_mut(&current_ptr)
            .ok_or(JsonEditError::InvalidPosition(Pointer::new(&current_ptr)))?;

        match current {
            Value::Array(arr) => {
                arr.push(value);
                Ok(self)
            }
            Value::Object(obj) => {
                // For objects, generate a new unique key
                let new_key = format!("item_{}", obj.len());
                obj.insert(new_key, value);
                Ok(self)
            }
            _ => Err(JsonEditError::ExpectedArrayOrObject(Pointer::new(
                &current_ptr,
            ))),
        }
    }

    fn construct_path_to(&mut self, target_ptr: &str) -> Result<(), JsonEditError> {
        let pointer = Pointer::new(target_ptr);
        let segments: Vec<String> = pointer.segments().collect();

        let mut current_path = String::new();

        for (idx, segment) in segments.iter().enumerate() {
            let parent_path = current_path.clone();
            current_path = if current_path.is_empty() {
                format!("/{}", segment)
            } else {
                format!("{}/{}", current_path, segment)
            };

            // Check if this segment already exists
            if self.json().pointer(&current_path).is_some() {
                continue;
            }

            // Determine what to create based on the segment
            let new_value = if segments
                .get(idx + 1) // Check if parent segment exists
                .unwrap_or(&"".to_string()) // if not make it unparsable
                .parse::<usize>()
                .is_ok()
            {
                // Segment is a number, create an array
                Value::Array(vec![])
            } else {
                // Segment is a string, create an object
                Value::Object(serde_json::Map::new())
            };

            // Insert into parent
            if parent_path.is_empty() {
                // We're at root, replace it
                *self.json_mut() = new_value;
            } else {
                let parent = self
                    .json_mut()
                    .pointer_mut(&parent_path)
                    .ok_or(JsonEditError::InvalidPosition(Pointer::new(&parent_path)))?;

                match parent {
                    Value::Object(obj) => {
                        obj.insert(segment.to_string(), new_value);
                    }
                    Value::Array(arr) => {
                        if let Ok(idx) = segment.parse::<usize>() {
                            // Extend array if needed
                            while arr.len() <= idx {
                                arr.push(Value::Null);
                            }
                            arr[idx] = new_value;
                        } else {
                            return Err(JsonEditError::ArrayIndexOutOfBounce(Pointer::new(
                                &parent_path,
                            )));
                        }
                    }
                    _ => {
                        return Err(JsonEditError::ExpectedArrayOrObject(Pointer::new(
                            &parent_path,
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Inserts a value into an array at the cursor's current position.
    ///
    /// # Arguments
    /// * `value` - The value to insert.
    ///
    /// # Returns
    /// * `Ok(())` if insertion was successful.
    /// * `Err(())` if current position is not a valid array index.
    pub fn insert(&mut self, value: Value) -> Result<&mut Self, JsonEditError> {
        if self.is_root() {
            return Err(JsonEditError::RootInsert);
        }
        self.up();
        let parent_ptr = self.pointer().position().to_string();
        let key = self.pointer().get_tip();

        let parent = self
            .json_mut()
            .pointer_mut(&parent_ptr)
            .ok_or(JsonEditError::InvalidPosition(Pointer::new(&parent_ptr)))?;

        match parent {
            Value::Array(arr) => {
                if let Ok(idx) = key.parse::<usize>() {
                    if idx <= arr.len() {
                        arr.insert(idx, value);
                        return Ok(self);
                    }
                }
                Err(JsonEditError::ArrayIndexOutOfBounce(Pointer::new(
                    &parent_ptr,
                )))
            }
            Value::Object(obj) => {
                obj.insert(key.to_string(), value);
                Ok(self)
            }
            _ => Err(JsonEditError::ExpectedArrayOrObject(Pointer::new(
                &parent_ptr,
            ))),
        }
    }
}

impl Deref for JsonEditor {
    type Target = JsonCursor;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl DerefMut for JsonEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cursor
    }
}
