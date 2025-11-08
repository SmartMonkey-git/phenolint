#![allow(dead_code)]
use crate::json::error::JsonEditError;
use crate::json::{JsonCursor, Pointer};
use serde_json::Value;
use std::ops::{Deref, DerefMut};

pub(crate) struct JsonEditor {
    cursor: JsonCursor,
}

impl JsonEditor {
    pub fn new(cursor: Value) -> Self {
        Self {
            cursor: JsonCursor::new(cursor),
        }
    }

    pub fn export(&self) -> Result<String, JsonEditError> {
        Ok(serde_json::to_string_pretty(self.json())?)
    }

    /// Replaces the value at the cursor's current position with a new value.
    ///
    /// # Arguments
    /// * `new_value` - The value to insert at the current position.
    ///
    /// # Returns
    /// * `Ok(())` if the replacement was successful.
    /// * `Err(())` if the current pointer doesn't point to a valid location.
    pub fn set_value(&mut self, new_value: Value) -> Result<(), JsonEditError> {
        let parsed = match &new_value {
            Value::String(s) => Self::parse_string(s).unwrap_or(new_value),
            _ => new_value,
        };

        if let Some(target) = self.current_value_mut() {
            *target = parsed;
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
        let key = self.pointer().get_tip();
        self.up();
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
        let parsed = match &value {
            Value::String(s) => Self::parse_string(s).unwrap_or(value),
            _ => value,
        };

        let current_ptr = self.pointer().position().to_string();

        let current = self.json_mut().pointer_mut(&current_ptr);

        if current.is_none() && construct_path {
            self.construct_path_to(&current_ptr)?;
        }

        let current = self
            .json_mut()
            .pointer_mut(&current_ptr)
            .ok_or(JsonEditError::InvalidPosition(Pointer::new(&current_ptr)))?;

        match current {
            Value::Array(arr) => {
                arr.push(parsed);
                Ok(self)
            }
            Value::Object(obj) => {
                if let Value::Object(obj_value) = parsed {
                    obj_value.iter().for_each(|(key, value)| {
                        obj.insert(key.to_owned(), value.to_owned());
                    });
                }

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

            if self.json().pointer(&current_path).is_some() {
                continue;
            }

            let new_value = if segments
                .get(idx + 1)
                .unwrap_or(&"".to_string())
                .parse::<usize>()
                .is_ok()
            {
                Value::Array(vec![])
            } else {
                Value::Object(serde_json::Map::new())
            };

            if parent_path.is_empty() {
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

    /// Returns a mutable reference to the JSON value at the cursor's current position.
    ///
    /// # Returns
    /// * `Some(&mut Value)` if the pointer resolves to a valid location.
    /// * `None` if the pointer path does not exist.
    fn current_value_mut(&mut self) -> Option<&mut Value> {
        let position = self.pointer().position().to_string();
        self.cursor.json_mut().pointer_mut(&position)
    }

    fn parse_value(value: &Value) -> Option<Value> {
        if let Value::String(s) = &value {
            Self::parse_string(s)
        } else {
            None
        }
    }

    fn parse_string(s: &str) -> Option<Value> {
        serde_json::from_str::<Value>(s).ok()
    }
}

impl Deref for JsonEditor {
    type Target = JsonCursor;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl From<JsonCursor> for JsonEditor {
    fn from(cursor: JsonCursor) -> Self {
        JsonEditor { cursor }
    }
}

impl DerefMut for JsonEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cursor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use serde_json::json;

    #[fixture]
    fn test_json() -> Value {
        json!({
            "id": 101,
            "name": "Alice Johnson",
            "email": "alice.johnson@example.com",
            "isActive": true,
            "roles": ["admin", "editor"],
            "profile": {
                "age": 29,
                "city": "New York",
                "preferences": {
                    "theme": "dark",
                    "notifications": true
                }
            },
            "is_active": true,
        })
    }

    #[rstest]
    #[case(Value::String("false".to_string()), Pointer::new("is_active"), Value::Bool(false))]
    #[case(Value::Number(500.into()), Pointer::new("id"), Value::Number(500.into()))]
    #[case(Value::String("guest".to_string()), Pointer::new("/roles/0"), Value::String("guest".to_string()))]
    #[case(Value::String("good".to_string()), Pointer::new("profile/preferences"), Value::String("good".to_string()))]
    fn test_set_value(
        test_json: Value,
        #[case] new_value: Value,
        #[case] pointer: Pointer,
        #[case] expected_val: Value,
    ) {
        let mut editor = JsonEditor::new(test_json);

        editor.point_to(&pointer);
        editor.set_value(new_value).unwrap();
        assert_eq!(editor.current_value().unwrap(), &expected_val);
    }

    #[rstest]
    fn test_delete_from_obj(test_json: Value) {
        let mut editor = JsonEditor::new(test_json);
        let to_delete = "profile";

        editor.down(to_delete);
        assert!(editor.delete().is_some());

        assert!(!editor.root().peek().contains(&to_delete.to_string()));
        assert_eq!(editor.down(to_delete).current_value(), None);
    }

    #[rstest]
    fn test_delete_from_array(test_json: Value) {
        let n_entries = test_json.get("roles").unwrap().as_array().unwrap().len();
        let other_entry = test_json
            .get("roles")
            .unwrap()
            .as_array()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let mut editor = JsonEditor::new(test_json);
        let to_delete = "roles/0";

        editor.down(to_delete);
        assert!(editor.delete().is_some());
        assert_eq!(
            editor.current_value().unwrap().as_array().unwrap().len(),
            n_entries - 1,
        );
        assert_eq!(&other_entry, "editor");
    }

    #[rstest]
    fn test_push_to_root(test_json: Value) {
        let mut editor = JsonEditor::new(test_json);

        let zip_code_key = "zip_code";
        let authenticated_key = "authenticated";
        let zip_value = 12345;
        let auth_value = true;
        let new_value = json!({zip_code_key: zip_value, authenticated_key: auth_value});

        editor.push(new_value, false).unwrap();
        assert!(editor.peek().contains(&zip_code_key.to_string()));
        assert!(editor.peek().contains(&authenticated_key.to_string()));
        assert_eq!(
            editor
                .down(zip_code_key)
                .current_value()
                .unwrap()
                .as_number()
                .unwrap()
                .as_i64()
                .unwrap(),
            zip_value,
        );
        assert_eq!(
            editor
                .root()
                .down(authenticated_key)
                .current_value()
                .unwrap()
                .as_bool()
                .unwrap(),
            auth_value,
        );
    }

    #[rstest]
    fn test_push_to_obj(test_json: Value) {
        let mut editor = JsonEditor::new(test_json);

        let zip_code_key = "zip_code";
        let authenticated_key = "authenticated";
        let zip_value = 12345;
        let auth_value = true;

        let new_value = json!({zip_code_key: zip_value, authenticated_key: auth_value});

        editor.down("profile");
        editor.push(new_value, false).unwrap();
        assert!(editor.peek().contains(&zip_code_key.to_string()));
        assert!(editor.peek().contains(&authenticated_key.to_string()));
        assert_eq!(
            editor
                .down(zip_code_key)
                .current_value()
                .unwrap()
                .as_number()
                .unwrap()
                .as_i64()
                .unwrap(),
            zip_value,
        );
        assert_eq!(
            editor
                .up()
                .down(authenticated_key)
                .current_value()
                .unwrap()
                .as_bool()
                .unwrap(),
            auth_value,
        );
    }

    #[rstest]
    fn test_push_to_obj_with_path_construction(test_json: Value) {
        let mut editor = JsonEditor::new(test_json);
        let new_path = "profile/some/new/path";
        let zip_code_key = "zip_code";
        let authenticated_key = "authenticated";
        let zip_value = 12345;
        let auth_value = true;

        let new_value = json!({zip_code_key: zip_value, authenticated_key: auth_value});

        editor.point_to(&Pointer::new(new_path));
        editor.push(new_value, true).unwrap();
        assert!(editor.peek().contains(&zip_code_key.to_string()));
        assert!(editor.peek().contains(&authenticated_key.to_string()));
        assert_eq!(
            editor
                .down(zip_code_key)
                .current_value()
                .unwrap()
                .as_number()
                .unwrap()
                .as_i64()
                .unwrap(),
            zip_value,
        );
        assert_eq!(
            editor
                .up()
                .down(authenticated_key)
                .current_value()
                .unwrap()
                .as_bool()
                .unwrap(),
            auth_value,
        );
    }
}
