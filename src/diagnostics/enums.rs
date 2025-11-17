#[derive(Debug)]
pub enum PhenopacketData {
    Text(String),
    Binary(Vec<u8>),
}
