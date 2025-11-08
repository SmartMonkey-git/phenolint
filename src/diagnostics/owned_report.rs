use ariadne::Report;
use std::ops::Range;

#[derive(Debug)]
pub struct OwnedReport {
    report: Report<'static, (&'static str, Range<usize>)>,
}

impl OwnedReport {
    pub fn new(report: Report<'static, (&'static str, Range<usize>)>) -> Self {
        Self { report }
    }
    pub fn report(&self) -> &Report<'static, (&'static str, Range<usize>)> {
        &self.report
    }

    pub fn into_inner(self) -> Report<'static, (&'static str, Range<usize>)> {
        self.report
    }
}
