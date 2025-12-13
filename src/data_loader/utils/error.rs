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
    #[snafu(display("Invalid Texture Load Type: `{load_type}`. (Position: {pos})"))]
    InvalidTextureLoadType { load_type: u32, pos: u64 },
    #[snafu(display("Invalid sprite sep masks type: `{mask_type}`. (Position: {pos})"))]
    InvalidSepMasksType { mask_type: u32, pos: u64 },
    #[snafu(display("Outdated sprite type: `{sprite_type}`. (Position: {pos})"))]
    OutdatedSpriteType { sprite_type: u32, pos: u64 },
    #[snafu(display("Outdated sprite version: `{sprite_version}`. (Position: {pos})"))]
    OutdatedSpriteVersion { sprite_version: u32, pos: u64 },
    #[snafu(display("Invalid playback speed type: `{speed_type}`. (Position: {pos})"))]
    InvalidPlaybackSpeedType { speed_type: u32, pos: u64 },
    #[snafu(display("Invalid Tile Mode: `{mode}`. (Position: {pos})"))]
    InvalidTileMode { mode: u32, pos: u64 },
    #[snafu(display("{message}. (Position: {pos})"))]
    DebugError { message: String, pos: u64 },
    #[snafu(display("Invalid room flags: `{flag}`. (Position: {pos})"))]
    InvalidRoomFlags { pos: u64, flag: u32 },
    #[snafu(display("Invalid layer kind: `{kind}`. (Position: {pos})"))]
    InvalidLayerKind { pos: u64, kind: u32 },
    #[snafu(display("Invalid effect property kind: `{kind}`. (Position: {pos})"))]
    InvalidEffectPropertyKind { pos: u64, kind: u32 },
    #[snafu(display("Invalid collision shape kind: `{kind}`. (Position: {pos})"))]
    InvalidCollisionShapeKind { pos: u64, kind: u32 },
    #[snafu(display(
        "Invalid event array length. Got `{actual}` expected `{correct}`. (Position: {pos})"
    ))]
    InvalidEventArrayLength {
        pos: u64,
        actual: usize,
        correct: usize,
    },
}
