use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{Group, Renderer};
use std::fmt::{Display, Formatter};

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

impl Display for OwnedReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        let display_str = renderer.render(&[self.report()]);
        write!(f, "{}", display_str)
    }
}
