use crate::enums::Patch;
use crate::error::PatchingError;
use crate::json::JsonEditor;
use crate::json::json_cursor::JsonCursor;
use serde_json::json;
use std::cmp::Ordering;

pub struct Patcher;

impl Patcher {
    pub fn patch(&self, phenostr: &str, patches: Vec<&Patch>) -> Result<String, PatchingError> {
        let mut cursor = JsonCursor::new(serde_json::from_str(phenostr)?);

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
        cursor: &mut JsonCursor,
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

    fn apply(cursor: JsonCursor, patches: Vec<Patch>) -> Result<String, PatchingError> {
        let mut editor = JsonEditor::from(cursor);

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
