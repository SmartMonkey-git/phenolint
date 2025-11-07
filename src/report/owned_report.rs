use annotate_snippets::Group;

#[derive(Clone, Debug)]
pub struct OwnedReport {
    report: Group<'static>,
}

impl OwnedReport {
    pub fn new(report: Group<'static>) -> Self {
        Self { report }
    }
    pub fn report(&self) -> Group<'static> {
        self.report.clone()
    }
}
