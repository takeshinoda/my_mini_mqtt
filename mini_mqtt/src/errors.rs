use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParserError(String),
    ProtocolError(String),
    MalformedPacket(String),
    Common(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "I/O error in the MQTT codec: {}", err),
            Error::ParserError(msg) => write!(f, "Invalid data for decoding: {}", msg),
            Error::MalformedPacket(msg) => write!(f, "Malformed packet: {}", msg),
            Error::ProtocolError(msg) => write!(f, "Protocol Error packet: {}", msg),
            Error::Common(msg) => write!(f, "Error in the MQTT codec: {}", msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<nom::error::Error<&[u8]>> for Error {
    fn from(err: nom::error::Error<&[u8]>) -> Self {
        Error::ParserError(format ! ("Parsing error: {:?}", err))
    }
}

impl <I> From<nom::Err<(I, nom::error::ErrorKind)>> for Error {
    fn from(err: nom::Err<(I, nom::error::ErrorKind)>) -> Self {
        match err {
            nom::Err::Incomplete(needed) => Error::ParserError(
                format!("Parsing incomplete, needed: {:?}", needed),
            ),
            nom::Err::Error((_, kind)) | nom::Err::Failure((_, kind)) => Error::ParserError(
                format!(
                    "Parsing error, kind: {:?}",
                    kind
                ),
            )
        }
    }
}

impl From<Error> for nom::Err<nom::error::Error<&[u8]>> {
    fn from(_: Error) -> Self {
        nom::Err::Failure(nom::error::Error::new(b"", nom::error::ErrorKind::Verify))
    }
}
