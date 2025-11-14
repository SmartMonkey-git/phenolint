use codespan_reporting::files::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportParseError {
    #[error(transparent)]
    StringParsing(Error),
    #[error(transparent)]
    Emit(Error),
}
