use phenolint::diagnostics::{LintReport, ReportParser};
use phenolint::rules::interpretation::disease_consistency_rule::DiseaseConsistencyRule;
use phenolint::traits::RuleCheck;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{Diagnosis, Disease, Interpretation, OntologyClass};

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

    let diseases = vec![
        create_disease(disease1.clone()),
        create_disease(disease2.clone()),
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
    let mut report = LintReport::default();
    // HERE
    DiseaseConsistencyRule.check(
        serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
        &mut report,
    );

    let finding = report.findings.first().unwrap();

    ReportParser::emit(finding.violation().report());
    let parsed_report = ReportParser::parse(finding.violation().report());

    let violations = report.violations();
}
#[test]
fn test_hallo_world() {
    main()
}
