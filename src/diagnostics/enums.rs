#[derive(Debug, PartialEq)]
pub enum PhenopacketData {
    Text(String),
    Binary(Vec<u8>),
}
