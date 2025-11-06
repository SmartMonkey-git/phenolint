use once_cell::sync::OnceCell;
use ontolius::io::OntologyLoaderBuilder;
use ontolius::ontology::csr::FullCsrOntology;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct LinterContext {
    hpo_path: Option<PathBuf>,
    hpo: OnceCell<Option<Arc<FullCsrOntology>>>,
}

impl LinterContext {
    pub fn new(hpo_path: Option<PathBuf>) -> Self {
        LinterContext {
            hpo_path,
            hpo: OnceCell::default(),
        }
    }
    pub fn hpo(&mut self) -> Option<Arc<FullCsrOntology>> {
        let path = self.hpo_path.as_ref()?;

        self.hpo
            .get_or_init(|| {
                let loader = OntologyLoaderBuilder::new().obographs_parser().build();
                let ontology: Option<FullCsrOntology> = loader.load_from_path(path.clone()).ok();
                if let Some(ont) = ontology {
                    Some(Arc::new(ont))
                } else {
                    None
                }
            })
            .clone()
    }
}
