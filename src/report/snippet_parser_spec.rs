#[allow(dead_code)]
pub struct SnippetParserSpec<T> {
    pub path: String,
    pub line_start: usize,
    pub markers: Vec<T>,
    pub fold: bool,
}
