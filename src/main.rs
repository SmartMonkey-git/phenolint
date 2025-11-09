use phenolint::{Lint, Phenolinter};
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{Diagnosis, Disease, Interpretation, OntologyClass};
use std::path::PathBuf;

fn create_ontology_class(id: &str, label: &str) -> OntologyClass {
    OntologyClass {
        id: id.to_string(),
        label: label.to_string(),
    }
}

fn create_disease(term: OntologyClass) -> Disease {
    Disease {
        term: Some(term),
        ..Default::default()
    }
}

fn create_interpretation(disease: Option<OntologyClass>) -> Interpretation {
    Interpretation {
        diagnosis: disease.map(|d| Diagnosis {
            disease: Some(d),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn main() {
    let disease1 = create_ontology_class("MONDO:0007254", "Breast Cancer");
    let disease2 = create_ontology_class("MONDO:0005148", "Diabetes");
    let disease3 = create_ontology_class("MONDO:0005015", "Hypertension");
    let disease4 = create_ontology_class("MONDO_0005016", "Some Disease");

    let diseases = vec![
        create_disease(disease1.clone()),
        create_disease(disease2.clone()),
        create_disease(disease4.clone()),
    ];
    let interpretations = vec![
        create_interpretation(Some(disease1)),
        create_interpretation(Some(disease3.clone())),
    ];

    let phenopacket = Phenopacket {
        diseases,
        interpretations,
        ..Default::default()
    };

    let mut linter = Phenolinter::try_from(PathBuf::from(
        "/Users/rouvenreuter/Documents/Projects/phenolint/assets/phenolint.toml",
    ))
    .unwrap();
    let phenostr = serde_json::to_string_pretty(&phenopacket).unwrap();
    let lint_res = linter.lint(phenostr.as_str(), true, false);

    match lint_res {
        Ok(report) => {
            println!("{}", report.patched_phenopacket.unwrap());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            println!("Integration test failed!");
            std::process::exit(1);
        }
    }
}
