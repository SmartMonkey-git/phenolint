use crate::new::patches::patch_registry::PatchRegistry;
use inventory;

pub struct PatchRegistration {
    register: fn(&mut PatchRegistry),
}

inventory::collect!(PatchRegistration);

impl PatchRegistry {
    pub fn with_all_patches() -> Self {
        let mut registry = Self::new();

        for registration in inventory::iter::<PatchRegistration> {
            (registration.register)(&mut registry);
        }

        registry
    }
}
