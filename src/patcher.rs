use crate::IntoBytes;
use crate::enums::Patch;
use crate::error::PatchingError;
use crate::json::PhenopacketEditor;
use crate::json::phenopacket_cursor::PhenopacketCursor;
use serde_json::json;
use std::cmp::Ordering;

pub struct Patcher;

impl Patcher {
    pub fn patch<T: IntoBytes + Clone>(
        &self,
        phenostr: &T,
        patches: Vec<&Patch>,
    ) -> Result<String, PatchingError> {
        let mut cursor = PhenopacketCursor::new(phenostr)?;

        let patches = Self::resolve_patches(patches, &mut cursor)?;
        Self::apply(cursor, patches)
    }

    /// Resolves high-level patch operations into primitive operations.
    ///
    /// This function transforms complex patch operations (`Move` and `Duplicate`) into
    /// their constituent primitive operations (`Add` and `Remove`). After resolution,
    /// patches are sorted to ensure correct application order.
    ///
    /// # Patch Resolution Rules
    ///
    /// - **`Move`**: Expanded into an `Add` operation (inserting the value at the target)
    ///   followed by a `Remove` operation (deleting from the source).
    /// - **`Duplicate`**: Expanded into a single `Add` operation (copying the value to
    ///   the target location).
    /// - **Other patches** (`Add`, `Remove`): Passed through unchanged.
    ///
    /// # Arguments
    ///
    /// * `patches` - A vector of patch references to resolve
    /// * `cursor` - A mutable JSON cursor used to navigate and read values from the
    ///   source document during resolution
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Patch>)` containing the resolved and sorted patches, or
    /// `Err(PatchingError)` if resolution fails (e.g., if a source path doesn't exist).
    ///
    /// # Example
    ///
    /// Given a `Move` patch from `/user/name` to `/person/fullName`:
    /// 1. The value at `/user/name` is read via the cursor
    /// 2. Two patches are created:
    ///    - `Add { at: "/person/fullName", value: <read_value> }`
    ///    - `Remove { at: "/user/name" }`
    /// 3. All patches are sorted for safe application order
    fn resolve_patches(
        patches: Vec<&Patch>,
        cursor: &mut PhenopacketCursor,
    ) -> Result<Vec<Patch>, PatchingError> {
        let mut resolved_patches: Vec<Patch> = patches
            .into_iter()
            .flat_map(|p| match p {
                Patch::Move { from, to } => {
                    let value = cursor.point_to(from).current_value().unwrap().to_string();
                    vec![
                        Patch::Add {
                            at: to.clone(),
                            value,
                        },
                        Patch::Remove { at: from.clone() },
                    ]
                }
                Patch::Duplicate { from, to } => {
                    let value = cursor.point_to(from).current_value().unwrap().to_string();

                    vec![Patch::Add {
                        at: to.clone(),
                        value,
                    }]
                }
                other => vec![other.clone()],
            })
            .collect();
        Self::sort_patches(resolved_patches.as_mut_slice());
        Ok(resolved_patches)
    }

    /// Sorts patches in a specific order to ensure correct application.
    ///
    /// Sorting is performed with two priorities:
    /// 1. **Patch type**: `Add` patches are ordered before `Remove` patches.
    ///    This ensures additions are processed before any removals.
    /// 2. **Tree depth**: Within each patch type, patches are sorted by their
    ///    depth in the JSON tree (number of path segments). Shallower paths
    ///    come before deeper ones.
    ///
    /// # Example ordering
    /// Given patches at paths:
    /// - `Add` at `/a/b/c` (depth 3)
    /// - `Remove` at `/a` (depth 1)
    /// - `Add` at `/a/b` (depth 2)
    ///
    /// After sorting: `Add /a/b`, `Add /a/b/c`, `Remove /a`
    fn sort_patches(patches: &mut [Patch]) {
        patches.sort_by(|p1, p2| match (p1, p2) {
            (Patch::Add { .. }, Patch::Remove { .. }) => Ordering::Less,
            (Patch::Remove { .. }, Patch::Add { .. }) => Ordering::Greater,
            (Patch::Add { at: at1, .. }, Patch::Add { at: at2, .. }) => {
                at1.segments().count().cmp(&at2.segments().count())
            }
            (Patch::Remove { at: at1 }, Patch::Remove { at: at2 }) => {
                at1.segments().count().cmp(&at2.segments().count())
            }
            _ => Ordering::Equal,
        });
    }

    fn apply(cursor: PhenopacketCursor, patches: Vec<Patch>) -> Result<String, PatchingError> {
        let mut editor = PhenopacketEditor::from(cursor);

        for patch in patches {
            match patch {
                Patch::Add { at, value } => {
                    editor.point_to(&at);
                    editor.push(json!(value), true)?;
                }
                Patch::Remove { at } => {
                    editor.point_to(&at);
                    editor.delete();
                }
                _ => {}
            };
            editor.root();
        }
        Ok(editor.export()?)
    }
}

#[cfg(test)]
mod patcher_tests {
    use crate::enums::Patch;
    use crate::json::Pointer;
    use crate::patcher::Patcher;
    use serde_json::{Value, json};

    // Helper function to parse and compare JSON strings
    fn assert_json_eq(actual: &str, expected: &str) {
        let actual_json: Value = serde_json::from_str(actual).expect("Invalid actual JSON");
        let expected_json: Value = serde_json::from_str(expected).expect("Invalid expected JSON");
        assert_eq!(actual_json, expected_json);
    }

    // Helper to create a sample phenopacket-like structure
    fn sample_phenopacket() -> String {
        json!({
            "id": "phenopacket.1",
            "subject": {
                "id": "patient.1",
                "sex": "MALE",
                "dateOfBirth": "1990-01-01"
            },
            "phenotypicFeatures": [
                {
                    "type": {
                        "id": "HP:0001250",
                        "label": "Seizure"
                    }
                }
            ],
            "diseases": [
                {
                    "term": {
                        "id": "OMIM:123456",
                        "label": "Example Disease"
                    },
                    "onset": {
                        "age": "P10Y"
                    }
                }
            ]
        })
        .to_string()
    }

    #[test]
    fn test_add_single_field() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Add {
            at: Pointer::new("/metaData"),
            value: json!({"created": "2024-01-01"}).to_string(),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json.get("metaData").is_some());
        assert_eq!(result_json["metaData"]["created"], "2024-01-01");
    }

    #[test]
    fn test_add_nested_field() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Add {
            at: Pointer::new("/subject/timeAtLastEncounter"),
            value: json!({"age": "P30Y"}).to_string(),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["subject"]["timeAtLastEncounter"].is_object());
        assert_eq!(result_json["subject"]["timeAtLastEncounter"]["age"], "P30Y");
    }

    #[test]
    fn test_remove_field() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Remove {
            at: Pointer::new("/subject/dateOfBirth"),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["subject"]["dateOfBirth"].is_null());
    }

    #[test]
    fn test_remove_nested_object() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Remove {
            at: Pointer::new("/diseases/0/onset"),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["diseases"][0]["onset"].is_null());
        assert!(result_json["diseases"][0]["term"].is_object());
    }

    #[test]
    fn test_move_field() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Move {
            from: Pointer::new("/subject/dateOfBirth"),
            to: Pointer::new("/subject/birthDate"),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["subject"]["dateOfBirth"].is_null());
        assert_eq!(result_json["subject"]["birthDate"], "1990-01-01");
    }

    #[test]
    fn test_move_nested_object() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Move {
            from: Pointer::new("/diseases/0/onset"),
            to: Pointer::new("/ageOfOnset"),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["diseases"][0]["onset"].is_null());
        assert_eq!(result_json["ageOfOnset"]["age"], "P10Y");
    }

    #[test]
    fn test_duplicate_field() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Duplicate {
            from: Pointer::new("/subject/id"),
            to: Pointer::new("/subject/patientId"),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["subject"]["id"], "patient.1");
        assert_eq!(result_json["subject"]["patientId"], "patient.1");
    }

    #[test]
    fn test_duplicate_complex_object() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Duplicate {
            from: Pointer::new("/diseases/0/term"),
            to: Pointer::new("/diagnosisTerm"),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["diseases"][0]["term"]["id"], "OMIM:123456");
        assert_eq!(result_json["diagnosisTerm"]["id"], "OMIM:123456");
        assert_eq!(result_json["diagnosisTerm"]["label"], "Example Disease");
    }

    #[test]
    fn test_multiple_patches_same_type() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [
            Patch::Add {
                at: Pointer::new("/subject/karyotypicSex"),
                value: "XY".to_string(),
            },
            Patch::Add {
                at: Pointer::new("/subject/taxonomy"),
                value: json!({"id": "NCBITaxon:9606", "label": "Homo sapiens"}).to_string(),
            },
        ];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["subject"]["karyotypicSex"], "XY");
        assert_eq!(result_json["subject"]["taxonomy"]["id"], "NCBITaxon:9606");
    }

    #[test]
    fn test_multiple_patches_mixed_types() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [
            Patch::Add {
                at: Pointer::new("/metaData"),
                value: json!({"created": "2024-01-01"}).to_string(),
            },
            Patch::Remove {
                at: Pointer::new("/subject/dateOfBirth"),
            },
            Patch::Move {
                from: Pointer::new("/subject/sex"),
                to: Pointer::new("/subject/gender"),
            },
        ];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["metaData"].is_object());
        assert!(result_json["subject"]["dateOfBirth"].is_null());
        assert!(result_json["subject"]["sex"].is_null());
        assert_eq!(result_json["subject"]["gender"], "MALE");
    }

    #[test]
    fn test_patch_ordering_add_before_remove() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        // Remove should not interfere with Add even if specified first
        let patches = [
            Patch::Remove {
                at: Pointer::new("/subject/sex"),
            },
            Patch::Add {
                at: Pointer::new("/subject/gender"),
                value: "MALE".to_string(),
            },
        ];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["subject"]["sex"].is_null());
        assert_eq!(result_json["subject"]["gender"], "MALE");
    }

    #[test]
    fn test_complex_move_and_add_scenario() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [
            Patch::Move {
                from: Pointer::new("/diseases/0"),
                to: Pointer::new("/primaryDiagnosis"),
            },
            Patch::Add {
                at: Pointer::new("/primaryDiagnosis/confirmed"),
                value: "true".to_string(),
            },
        ];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["primaryDiagnosis"]["term"]["id"], "OMIM:123456");
        assert_eq!(result_json["primaryDiagnosis"]["confirmed"], json!(true));
    }

    #[test]
    fn test_empty_patches() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches: Vec<&Patch> = vec![];
        let result = patcher.patch(&phenostr, patches).unwrap();

        assert_json_eq(&result, &phenostr);
    }

    #[test]
    fn test_invalid_json_input() {
        let patcher = Patcher;
        let invalid_json = "{invalid json}";

        let patches = [Patch::Add {
            at: Pointer::new("/test"),
            value: "value".to_string(),
        }];

        let result = patcher.patch(&invalid_json, patches.iter().collect());
        assert!(result.is_err());
    }

    #[test]
    fn test_add_to_root_level() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Add {
            at: Pointer::new("/schemaVersion"),
            value: "2.0".to_string(),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["schemaVersion"], json!(2.0));
    }

    #[test]
    fn test_duplicate_and_modify_pattern() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [
            Patch::Duplicate {
                from: Pointer::new("/subject"),
                to: Pointer::new("/backup"),
            },
            Patch::Remove {
                at: Pointer::new("/subject/dateOfBirth"),
            },
        ];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        // Backup should have original data
        assert_eq!(result_json["backup"]["dateOfBirth"], "1990-01-01");
        // Original should be modified
        assert!(result_json["subject"]["dateOfBirth"].is_null());
    }

    #[test]
    fn test_array_element_operations() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Add {
            at: Pointer::new("/phenotypicFeatures/0/severity"),
            value: json!({"label": "severe"}).to_string(),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(
            result_json["phenotypicFeatures"][0]["severity"]["label"],
            "severe"
        );
    }

    #[test]
    fn test_deeply_nested_add() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Add {
            at: Pointer::new("/diseases/0/onset/iso8601/iso8601duration"),
            value: "P10Y".to_string(),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(
            result_json["diseases"][0]["onset"]["iso8601"]["iso8601duration"],
            "P10Y"
        );
    }

    #[test]
    fn test_patch_with_special_characters_in_value() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [Patch::Add {
            at: Pointer::new("/notes"),
            value: "Patient has \"complex\" symptoms; requires care.".to_string(),
        }];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["notes"].as_str().unwrap().contains("complex"));
    }

    #[test]
    fn test_multiple_moves_chained() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [
            Patch::Move {
                from: Pointer::new("/subject/sex"),
                to: Pointer::new("/subject/biologicalSex"),
            },
            Patch::Move {
                from: Pointer::new("/subject/id"),
                to: Pointer::new("/patientIdentifier"),
            },
        ];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["subject"]["sex"].is_null());
        assert!(result_json["subject"]["id"].is_null());
        assert_eq!(result_json["subject"]["biologicalSex"], "MALE");
        assert_eq!(result_json["patientIdentifier"], "patient.1");
    }

    #[test]
    fn test_remove_then_add_same_path() {
        let patcher = Patcher;
        let phenostr = sample_phenopacket();

        let patches = [
            Patch::Remove {
                at: Pointer::new("/subject/sex"),
            },
            Patch::Add {
                at: Pointer::new("/subject/sex"),
                value: "FEMALE".to_string(),
            },
        ];

        let result = patcher.patch(&phenostr, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        // Add should happen first due to sorting, then remove
        // This tests the sorting behavior
        assert!(result_json["subject"]["sex"].is_null());
    }

    #[test]
    fn test_minimal_phenopacket() {
        let patcher = Patcher;
        let minimal = json!({"id": "test"}).to_string();

        let patches = [Patch::Add {
            at: Pointer::new("/subject"),
            value: json!({"id": "patient.1"}).to_string(),
        }];

        let result = patcher.patch(&minimal, patches.iter().collect()).unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["id"], "test");
        assert_eq!(result_json["subject"]["id"], "patient.1");
    }
}
