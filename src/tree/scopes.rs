use crate::tree::pointer::Pointer;
use crate::tree::querying::traits::ScopeDefinition;
use phenopackets::schema::v2::{Cohort, Family, Phenopacket};
use std::any::TypeId;
use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeLayer {
    Individual = 0,
    Aggregated = 1,
}

impl ScopeDefinition for Phenopacket {
    fn layer() -> ScopeLayer {
        ScopeLayer::Individual
    }

    fn partitioning_fields() -> &'static [&'static str] {
        &["members", "relatives", "proband"]
    }
}

impl ScopeDefinition for Family {
    fn layer() -> ScopeLayer {
        ScopeLayer::Aggregated
    }
}

impl ScopeDefinition for Cohort {
    fn layer() -> ScopeLayer {
        ScopeLayer::Aggregated
    }
}

pub(crate) struct ScopeMappings {
    scope_by_type_id: HashMap<TypeId, ScopeLayer>,
    boundaries: HashMap<&'static str, TypeId>,
    max_seen_scope: Cell<ScopeLayer>,
}

impl ScopeMappings {
    pub(crate) fn new() -> Self {
        let mut scope_map = Self {
            max_seen_scope: Cell::from(ScopeLayer::Individual),
            scope_by_type_id: HashMap::new(),
            boundaries: HashMap::new(),
        };

        scope_map.register::<Phenopacket>();
        scope_map.register::<Cohort>();
        scope_map.register::<Family>();

        scope_map
    }

    fn register<NodeType: ScopeDefinition + 'static>(&mut self) {
        let type_id = TypeId::of::<NodeType>();
        self.scope_by_type_id.insert(type_id, NodeType::layer());

        for b_field in NodeType::partitioning_fields() {
            self.boundaries.insert(b_field, type_id);
        }
    }

    pub fn get_scope(&self, type_id: &TypeId) -> Option<ScopeLayer> {
        self.scope_by_type_id.get(type_id).copied()
    }

    pub fn is_scope_boundary(&self, type_id: &TypeId) -> bool {
        self.scope_by_type_id.contains_key(type_id)
    }

    pub fn derive_scope(&self, path: &Pointer, type_id: &TypeId) -> ScopeLayer {
        if let Some(scope) = self.get_scope(type_id) {
            let current_max = self.max_seen_scope.get();
            self.max_seen_scope.set(current_max.max(scope));
        }

        let segments: Vec<Cow<str>> = path.iter_segments().collect();

        for segment in segments.iter().rev() {
            if let Some(boundary_type_id) = self.boundaries.get(segment.as_ref()) {
                return self
                    .scope_by_type_id
                    .get(boundary_type_id)
                    .copied()
                    .expect("Configuration error: Boundary points to unknown scope");
            }
        }

        self.max_seen_scope.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phenopackets::schema::v2::core::Resource;

    #[test]
    fn test_derive_scope_single_pp() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Phenopacket>();

        assert_eq!(
            scope_map.derive_scope(&Pointer::at_root(), &type_id),
            ScopeLayer::Individual
        );
    }

    #[test]
    fn test_derive_scope_cohort() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Cohort>();

        assert_eq!(
            scope_map.derive_scope(&Pointer::at_root(), &type_id),
            ScopeLayer::Aggregated
        );
    }

    #[test]
    fn test_derive_scope_family() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Family>();

        assert_eq!(
            scope_map.derive_scope(&Pointer::at_root(), &type_id),
            ScopeLayer::Aggregated
        );
    }

    #[test]
    fn test_derive_scope_cohort_with_pp() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Cohort>();

        assert_eq!(
            scope_map.derive_scope(&Pointer::at_root(), &type_id),
            ScopeLayer::Aggregated
        );

        let type_id = TypeId::of::<Phenopacket>();

        assert_eq!(
            scope_map.derive_scope(
                &Pointer::from(
                    format!("/{}", Phenopacket::partitioning_fields().first().unwrap()).as_str()
                ),
                &type_id
            ),
            ScopeLayer::Individual
        );
    }

    #[test]
    fn test_derive_scope_cohort_with_random() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Cohort>();

        assert_eq!(
            scope_map.derive_scope(&Pointer::at_root(), &type_id),
            ScopeLayer::Aggregated
        );

        let type_id = TypeId::of::<Resource>();

        assert_eq!(
            scope_map.derive_scope(
                &Pointer::from(
                    format!(
                        "/{}/metaData/resources",
                        Phenopacket::partitioning_fields().first().unwrap()
                    )
                    .as_str()
                ),
                &type_id
            ),
            ScopeLayer::Individual
        );

        assert_eq!(
            scope_map.derive_scope(&Pointer::from("/resources"), &type_id),
            ScopeLayer::Aggregated
        );
    }
}
