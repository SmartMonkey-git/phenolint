#[derive(Debug, PartialEq, Clone)]
pub enum PhenopacketData {
    Text(String),
    Binary(Vec<u8>),
}
