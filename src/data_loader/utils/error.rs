use snafu::prelude::*;
use std::{io, string::FromUtf8Error};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DataLoadError {
    #[snafu(display("Mismatched chunk identifier at {pos}."))]
    UnexpectedIdent {
        pos: u64,
        expected: [u8; 4],
        actual: [u8; 4],
    },
    #[snafu(display("String Decode Error: `{source}`. (Position: {pos})"))]
    StringDecodeError { source: FromUtf8Error, pos: u64 },
    #[snafu(display("IO Error: `{source}`. (Position: {pos})"))]
    IOError { source: io::Error, pos: u64 },
    #[snafu(display("Escaped the chunk! At position {pos}, should be {loc}."))]
    ChunkEscapeError { pos: u64, loc: u64 },
}
