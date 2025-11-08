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

    pub fn push(&mut self, value: Value) -> Result<&mut Self, JsonEditError> {
        let current_ptr = self.pointer().position().to_string();

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
