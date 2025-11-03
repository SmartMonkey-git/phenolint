use once_cell::sync::Lazy;
use ontolius::io::OntologyLoaderBuilder;
use ontolius::ontology::csr::FullCsrOntology;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) static HPO: Lazy<Arc<FullCsrOntology>> = Lazy::new(|| init_ontolius("/Users/rouvenreuter/Library/Caches/phenoxtract/hp/2025-09-01_hp.json".into()));

fn init_ontolius(hpo_path: PathBuf) -> Arc<FullCsrOntology> {
    let loader = OntologyLoaderBuilder::new().obographs_parser().build();

    let ontolius = loader
        .load_from_path(hpo_path.clone())
        .expect("Unable to load ontology");
    Arc::new(ontolius)
}
