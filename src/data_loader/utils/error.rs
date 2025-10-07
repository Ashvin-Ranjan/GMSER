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
    #[snafu(display("GMSER does not currently support version {major}.{minor} (build {build}, release {release}"))]
    InvalidVersionError {
        major: u32,
        minor: u32,
        build: u32,
        release: u32,
    },
    #[snafu(display("Invalid sound flags: `{flag}`. (Position: {pos})"))]
    InvalidSoundFlags { pos: u64, flag: u32 },
    #[snafu(display("Invalid info flags: `{flag}`. (Position: {pos})"))]
    InvalidInfoFlags { pos: u64, flag: u32 },
    #[snafu(display("Unsupported option version: `{version}`. (Position: {pos})"))]
    UnsupportedOptionVerion { pos: u64, version: u32 },
    #[snafu(display("Invalid option flags: `{flag}`. (Position: {pos})"))]
    InvalidOptionFlags { pos: u64, flag: u64 },
    #[snafu(display("Invalid function classifications: `{classifications}`. (Position: {pos})"))]
    InvalidFunctionClassifications { pos: u64, classifications: u64 },
    #[snafu(display("Attempting to read a pointer at {pos}."))]
    InvalidReadError { pos: u64 },
    #[snafu(display("Invalid texture header at {pos}."))]
    InvalidTextureHeader { pos: u64, header: [u8; 8] },
    #[snafu(display("BZip2 Decompression Failure: `{source}`. (Position: {pos})"))]
    BZip2DecompressionError { source: io::Error, pos: u64 },
}
