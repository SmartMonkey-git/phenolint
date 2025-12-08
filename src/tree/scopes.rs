use crate::tree::pointer::Pointer;
use phenopackets::schema::v2::{Cohort, Family, Phenopacket};
use std::any::TypeId;
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ScopeLayer {
    Individual = 0,
    Aggregated = 1,
}

pub(crate) trait ScopeDefinition {
    fn layer() -> ScopeLayer;

    fn partitioning_fields() -> &'static [&'static str] {
        &[]
    }
}

impl ScopeDefinition for Phenopacket {
    fn layer() -> ScopeLayer {
        ScopeLayer::Individual
    }

    fn partitioning_fields() -> &'static [&'static str] {
        &["members", "relatives", "probands"]
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

    fn register<T: ScopeDefinition + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.scope_by_type_id.insert(type_id, T::layer());

        for b_field in T::partitioning_fields() {
            self.boundaries.insert(b_field, type_id);
        }
    }

    pub fn get_scope(&self, type_id: &TypeId) -> Option<ScopeLayer> {
        self.scope_by_type_id.get(type_id).copied()
    }

    pub fn is_scope_boundary(&self, type_id: &TypeId) -> bool {
        self.scope_by_type_id.contains_key(type_id)
    }

    pub fn derive_scope(&self, path: &str, type_id: &TypeId) -> ScopeLayer {
        if let Some(scope) = self.get_scope(type_id) {
            let current_max = self.max_seen_scope.get();
            self.max_seen_scope.set(current_max.max(scope));
        }

        let segments: Vec<_> = Pointer::new(path).segments().collect();
        for segment in segments.iter().rev() {
            if let Some(boundary_type_id) = self.boundaries.get(segment.as_str()) {
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

        assert_eq!(scope_map.derive_scope("", &type_id), ScopeLayer::Individual);
    }

    #[test]
    fn test_derive_scope_cohort() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Cohort>();

        assert_eq!(scope_map.derive_scope("", &type_id), ScopeLayer::Aggregated);
    }

    #[test]
    fn test_derive_scope_family() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Family>();

        assert_eq!(scope_map.derive_scope("", &type_id), ScopeLayer::Aggregated);
    }

    #[test]
    fn test_derive_scope_cohort_with_pp() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Cohort>();

        assert_eq!(scope_map.derive_scope("", &type_id), ScopeLayer::Aggregated);

        let type_id = TypeId::of::<Phenopacket>();

        assert_eq!(
            scope_map.derive_scope(
                &format!("/{}", Phenopacket::partitioning_fields().first().unwrap()),
                &type_id
            ),
            ScopeLayer::Individual
        );
    }

    #[test]
    fn test_derive_scope_cohort_with_random() {
        let scope_map = ScopeMappings::new();

        let type_id = TypeId::of::<Cohort>();

        assert_eq!(scope_map.derive_scope("", &type_id), ScopeLayer::Aggregated);

        let type_id = TypeId::of::<Resource>();

        assert_eq!(
            scope_map.derive_scope(
                &format!(
                    "/{}/metaData/resources",
                    Phenopacket::partitioning_fields().first().unwrap()
                ),
                &type_id
            ),
            ScopeLayer::Individual
        );

        assert_eq!(
            scope_map.derive_scope("/resources", &type_id),
            ScopeLayer::Aggregated
        );
    }
}
