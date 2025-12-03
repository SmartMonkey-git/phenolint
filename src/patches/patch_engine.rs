use crate::patches::enums::PatchInstruction;
use crate::patches::error::PatchingError;
use crate::patches::patch::Patch;
use serde_json::Value;
use std::cmp::Ordering;

#[derive(Debug, Default)]
pub struct PatchEngine;

impl PatchEngine {
    pub fn patch(&self, values: &Value, patches: Vec<&Patch>) -> Result<Value, PatchingError> {
        let patched_value = values.clone();
        let patch_instructions = Self::resolve_patches(patches, &patched_value)?;
        Self::apply(patched_value, patch_instructions)
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
        value: &Value,
    ) -> Result<Vec<PatchInstruction>, PatchingError> {
        let mut resolved_patches: Vec<PatchInstruction> = patches
            .into_iter()
            .flat_map(|p| {
                p.instructions()
                    .iter()
                    .flat_map(|instruction| match instruction {
                        PatchInstruction::Move { from, to } => {
                            let value = value.pointer(from.position()).unwrap();
                            vec![
                                PatchInstruction::Add {
                                    at: to.clone(),
                                    value: value.clone(),
                                },
                                PatchInstruction::Remove { at: from.clone() },
                            ]
                        }
                        PatchInstruction::Duplicate { from, to } => {
                            let value = value.pointer(from.position()).unwrap();

                            vec![PatchInstruction::Add {
                                at: to.clone(),
                                value: value.clone(),
                            }]
                        }
                        other => vec![other.clone()],
                    })
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
    fn sort_patches(patches: &mut [PatchInstruction]) {
        patches.sort_by(|p1, p2| match (p1, p2) {
            (PatchInstruction::Add { .. }, PatchInstruction::Remove { .. }) => Ordering::Less,
            (PatchInstruction::Remove { .. }, PatchInstruction::Add { .. }) => Ordering::Greater,
            (PatchInstruction::Add { at: at1, .. }, PatchInstruction::Add { at: at2, .. }) => {
                at1.segments().count().cmp(&at2.segments().count())
            }
            (PatchInstruction::Remove { at: at1 }, PatchInstruction::Remove { at: at2 }) => {
                at1.segments().count().cmp(&at2.segments().count())
            }
            _ => Ordering::Equal,
        });
    }

    fn apply(mut values: Value, patches: Vec<PatchInstruction>) -> Result<Value, PatchingError> {
        for patch in patches {
            let patch = patch.to_json_patch();
            json_patch::patch(&mut values, &patch)?;
        }
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::NonEmptyVec;
    use crate::patches::enums::PatchInstruction;
    use crate::patches::patch::Patch;
    use crate::patches::patch_engine::PatchEngine;
    use crate::tree::pointer::Pointer;
    use rstest::rstest;
    use serde_json::{Number, Value, json};

    // Helper to create a sample phenopacket-like structure
    fn sample_phenopacket() -> Value {
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
    }
    #[rstest]
    fn test_add_single_field() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();
        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Add {
                at: Pointer::new("/metaData"),
                value: json!({"created": "2024-01-01"}),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result.get("metaData").is_some());
        assert_eq!(result["metaData"]["created"], "2024-01-01");
    }

    #[test]
    fn test_add_nested_field() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Add {
                at: Pointer::new("/subject/timeAtLastEncounter"),
                value: json!({"age": "P30Y"}),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["subject"]["timeAtLastEncounter"].is_object());
        assert_eq!(result["subject"]["timeAtLastEncounter"]["age"], "P30Y");
    }

    #[test]
    fn test_remove_field() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Remove {
                at: Pointer::new("/subject/dateOfBirth"),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["subject"]["dateOfBirth"].is_null());
    }

    #[test]
    fn test_remove_nested_object() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Remove {
                at: Pointer::new("/diseases/0/onset"),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["diseases"][0]["onset"].is_null());
        assert!(result["diseases"][0]["term"].is_object());
    }

    #[test]
    fn test_move_field() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Move {
                from: Pointer::new("/subject/dateOfBirth"),
                to: Pointer::new("/subject/birthDate"),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["subject"]["dateOfBirth"].is_null());
        assert_eq!(result["subject"]["birthDate"], "1990-01-01");
    }

    #[test]
    fn test_move_nested_object() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Move {
                from: Pointer::new("/diseases/0/onset"),
                to: Pointer::new("/ageOfOnset"),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["diseases"][0]["onset"].is_null());
        assert_eq!(result["ageOfOnset"]["age"], "P10Y");
    }

    #[test]
    fn test_duplicate_field() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Duplicate {
                from: Pointer::new("/subject/id"),
                to: Pointer::new("/subject/patientId"),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert_eq!(result["subject"]["id"], "patient.1");
        assert_eq!(result["subject"]["patientId"], "patient.1");
    }

    #[test]
    fn test_duplicate_complex_object() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Duplicate {
                from: Pointer::new("/diseases/0/term"),
                to: Pointer::new("/diagnosisTerm"),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert_eq!(result["diseases"][0]["term"]["id"], "OMIM:123456");
        assert_eq!(result["diagnosisTerm"]["id"], "OMIM:123456");
        assert_eq!(result["diagnosisTerm"]["label"], "Example Disease");
    }

    #[test]
    fn test_multiple_patches_same_type() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Add {
                at: Pointer::new("/subject/karyotypicSex"),
                value: Value::String("XY".to_string()),
            },
            vec![PatchInstruction::Add {
                at: Pointer::new("/subject/taxonomy"),
                value: json!({"id": "NCBITaxon:9606", "label": "Homo sapiens"}),
            }],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert_eq!(result["subject"]["karyotypicSex"], "XY");
        assert_eq!(result["subject"]["taxonomy"]["id"], "NCBITaxon:9606");
    }

    #[test]
    fn test_multiple_patches_mixed_types() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Add {
                at: Pointer::new("/metaData"),
                value: json!({"created": "2024-01-01"}),
            },
            vec![
                PatchInstruction::Remove {
                    at: Pointer::new("/subject/dateOfBirth"),
                },
                PatchInstruction::Move {
                    from: Pointer::new("/subject/sex"),
                    to: Pointer::new("/subject/gender"),
                },
            ],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["metaData"].is_object());
        assert!(result["subject"]["dateOfBirth"].is_null());
        assert!(result["subject"]["sex"].is_null());
        assert_eq!(result["subject"]["gender"], "MALE");
    }

    #[test]
    fn test_patch_ordering_add_before_remove() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Remove {
                at: Pointer::new("/subject/sex"),
            },
            vec![PatchInstruction::Add {
                at: Pointer::new("/subject/gender"),
                value: Value::String("MALE".to_string()),
            }],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["subject"]["sex"].is_null());
        assert_eq!(result["subject"]["gender"], "MALE");
    }

    #[test]
    fn test_complex_move_and_add_scenario() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Move {
                from: Pointer::new("/diseases/0"),
                to: Pointer::new("/primaryDiagnosis"),
            },
            vec![PatchInstruction::Add {
                at: Pointer::new("/primaryDiagnosis/confirmed"),
                value: Value::Bool(true),
            }],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert_eq!(result["primaryDiagnosis"]["term"]["id"], "OMIM:123456");
        assert_eq!(result["primaryDiagnosis"]["confirmed"], json!(true));
    }

    #[test]
    fn test_empty_patches() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patches: Vec<&Patch> = vec![];
        let result = patcher.patch(&phenostr, patches).unwrap();

        assert_eq!(&result, &phenostr);
    }

    #[test]
    fn test_add_to_root_level() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Add {
                at: Pointer::new("/schemaVersion"),
                value: Value::Number(Number::from_f64(2.0f64).unwrap()),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert_eq!(result["schemaVersion"], json!(2.0));
    }

    #[test]
    fn test_duplicate_and_modify_pattern() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Duplicate {
                from: Pointer::new("/subject"),
                to: Pointer::new("/backup"),
            },
            vec![PatchInstruction::Remove {
                at: Pointer::new("/subject/dateOfBirth"),
            }],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        // Backup should have original data
        assert_eq!(result["backup"]["dateOfBirth"], "1990-01-01");
        // Original should be modified
        assert!(result["subject"]["dateOfBirth"].is_null());
    }

    #[test]
    fn test_array_element_operations() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Add {
                at: Pointer::new("/phenotypicFeatures/0/severity"),
                value: json!({"label": "severe"}),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert_eq!(
            result["phenotypicFeatures"][0]["severity"]["label"],
            "severe"
        );
    }

    #[test]
    fn test_deeply_nested_add() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Add {
                at: Pointer::new("/diseases/0/onset/iso8601"),
                value: json!({"iso8601duration": "P10Y"}),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert_eq!(
            result["diseases"][0]["onset"]["iso8601"]["iso8601duration"],
            "P10Y"
        );
    }

    #[test]
    fn test_patch_with_special_characters_in_value() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Add {
                at: Pointer::new("/notes"),
                value: Value::String(
                    "Patient has \"complex\" symptoms; requires care.".to_string(),
                ),
            },
            vec![],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["notes"].as_str().unwrap().contains("complex"));
    }

    #[test]
    fn test_multiple_moves_chained() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Move {
                from: Pointer::new("/subject/sex"),
                to: Pointer::new("/subject/biologicalSex"),
            },
            vec![PatchInstruction::Move {
                from: Pointer::new("/subject/id"),
                to: Pointer::new("/patientIdentifier"),
            }],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["subject"]["sex"].is_null());
        assert!(result["subject"]["id"].is_null());
        assert_eq!(result["subject"]["biologicalSex"], "MALE");
        assert_eq!(result["patientIdentifier"], "patient.1");
    }

    #[test]
    fn test_remove_then_add_same_path() {
        let patcher = PatchEngine;
        let phenostr = sample_phenopacket();

        let patch = Patch::new(NonEmptyVec::with_rest(
            PatchInstruction::Remove {
                at: Pointer::new("/subject/sex"),
            },
            vec![PatchInstruction::Add {
                at: Pointer::new("/subject/sex"),
                value: Value::String("FEMALE".to_string()),
            }],
        ));

        let result = patcher.patch(&phenostr, vec![&patch]).unwrap();

        assert!(result["subject"]["sex"].is_null());
    }

    #[test]
    fn test_minimal_phenopacket() {
        let patcher = PatchEngine;
        let minimal = json!({"id": "test"});

        let patch = Patch::new(NonEmptyVec::with_single_entry(PatchInstruction::Add {
            at: Pointer::new("/subject"),
            value: json!({"id": "patient.1"}),
        }));

        let result = patcher.patch(&minimal, vec![&patch]).unwrap();

        assert_eq!(result["id"], "test");
        assert_eq!(result["subject"]["id"], "patient.1");
    }
}
