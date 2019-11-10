use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    DeserializationError(String),
    TypeHintsRequired,
    InvalidAtom,
    InvalidBoolean,
    InvalidNumber,
    InvalidStringable,
    InvalidList,
    InvalidTuple,
    InvalidSequenceElement,
    ExpectedAtom,
    ExpectedBoolean,
    ExpectedBinary,
    ExpectedNumber,
    ExpectedChar,
    ExpectedStringable,
    ExpectedNil,
    ExpectedList,
    ExpectedTuple,
    ExpectedEnum,
    ExpectedMap,
    ExpectedStruct,
    ExpectedStructName,
    ExpectedStructValue,
    ExpectedUnitVariant,
    ExpectedNewtypeStruct,
    ExpectedNewtypeVariant,
    ExpectedTupleVariant,
    ExpectedStructVariant,
    SerializationError(String),
    InvalidVariantName,
    InvalidStructName,
    InvalidBinary,
    InvalidMap,
    InvalidStruct,
    InvalidStructKey,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;

        match &*self {
            DeserializationError(err) => err.as_str(),
            TypeHintsRequired => "Cannot deserialize any, type hints are required",
            InvalidAtom => "Failed to deserialize atom",
            InvalidBoolean => "Failed to deserialize boolean",
            InvalidNumber => "Failed to deserialize number",
            InvalidStringable => "Failed to deserialize term as an &str",
            InvalidList => "Failed to deserialize list",
            InvalidTuple => "Failed to deserialize tuple",
            InvalidSequenceElement => "Failed to deserialize sequence element",
            ExpectedAtom => "Expected to deserialize atom",
            ExpectedBoolean => "Expected to deserialize boolean",
            ExpectedBinary => "Expected to deserialize binary",
            ExpectedNumber => "Expected to deserialize number",
            ExpectedChar => "Expected to deserialize char",
            ExpectedStringable => "Expected to deserialize a UTF-8 stringable term",
            ExpectedNil => "Expected to deserialize nil",
            ExpectedList => "Expected to deserialize list",
            ExpectedTuple => "Expected to deserialize tuple",
            ExpectedEnum => "Expected to deserialize enum",
            ExpectedMap => "Expected to deserialize map",
            ExpectedStruct => "Expected to deserialize struct",
            ExpectedStructName => "Expected to deserialize struct name",
            ExpectedStructValue => "Expected to deserialize struct value",
            ExpectedUnitVariant => "Expected to deserialize unit variant",
            ExpectedNewtypeStruct => "Expected to deserialize newtype struct tuple",
            ExpectedNewtypeVariant => "Expected to deserialize newtype variant",
            ExpectedTupleVariant => "Expected to deserialize tuple variant",
            ExpectedStructVariant => "Expected to deserialize struct variant",
            SerializationError(err) => err.as_str(),
            InvalidVariantName => "Failed to serialize variant to atom or string",
            InvalidStructName => "Failed to serialize struct name to atom or string",
            InvalidBinary => "Failed to serialize binary",
            InvalidMap => "Failed to serialize map to NIF map",
            InvalidStruct => "Failed to serialize struct to NIF struct",
            InvalidStructKey => "Failed to serialize struct key",
        }
    }
}

impl From<Error> for crate::Error {
    fn from(err: Error) -> crate::Error {
        crate::Error::RaiseTerm(Box::new(String::from(err.to_string())))
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::SerializationError(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::DeserializationError(msg.to_string())
    }
}
