use std::fmt::{Display, Formatter};

#[doc(hidden)]
#[derive(PartialEq, Debug)]
pub enum InputTypes {
    Json,
    Yaml,
    Protobuf,
}

impl Display for InputTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let format_str = match self {
            InputTypes::Json => "json",
            InputTypes::Yaml => "yaml",
            InputTypes::Protobuf => "protobuf",
        };
        write!(f, "{}", format_str)
    }
}
