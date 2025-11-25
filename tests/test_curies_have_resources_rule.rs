mod common;

use crate::common::asserts::LintResultAssertSettings;
use crate::common::test_functions::run_rule_test;
use common::construction::minimal_valid_phenopacket;

#[test]
fn test_rule() {
    let pp = minimal_valid_phenopacket();

    let rule_id = "INTER002";
    let assert_settings = LintResultAssertSettings {
        rule_id,
        n_violations: 0,
        patched_phenopacket: None,
        patches: vec![],
        message_snippets: vec![],
    };
    run_rule_test(rule_id, &pp, assert_settings);
    /*
    The phenopacket in fact lacks several resources for the ontology terms such as:
    - `NCBITaxon`: subject > taxonomy > {"id": "NCBITaxon:9606", "label": "homo sapiens"}
    - `UBERON`: biosamples[0] > sampledTissue > {"id": "UBERON:0003403","label": "skin of forearm"}
    - `OMIM`: diseases[0] > term > {"id": "OMIM:101600", "label": "PFEIFFER SYNDROME"}
    */
    panic!("Passes now but it should not!")
}
