use phenopackets::schema::v2::core::OntologyClass;
use phenopackets::schema::v2::core::PhenotypicFeature;





#[derive(Clone, Debug)]
pub enum FixAction {
    Add {
        at: String,
        value: String,
    },
    Remove {
        at: String,
    },
    ///TODO: Unfolds to remove and add
    Move {
        from: String,
        to: String,
    },
    ///TODO: Converts to add
    Duplicate {
        from: String,
        to: String,
    },
}
