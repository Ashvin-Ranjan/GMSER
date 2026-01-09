//! This module contains the general error type for the errors which can occur while loading data.

use snafu::prelude::*;
use std::{io, string::FromUtf8Error};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]

/// This is the general error type for data-loading across [`crate::data_loader`].
pub enum DataLoadError {
    /// This indicates that a chunk has an identifier which was not expected
    /// # Notes
    /// - Since `[u8; 4]` does not implement [`std::fmt::Display`], the identifier cannot be displayed in the error message.
    #[snafu(display("Mismatched chunk identifier at {pos}."))]
    UnexpectedIdent {
        /// The position of the incorrect identifier.
        pos: u64,
        /// The expected identifier.
        expected: [u8; 4],
        /// The actual identifier.
        actual: [u8; 4],
    },

    /// This is a wrapper around [`std::string::FromUtf8Error`], primarily used in [`crate::data_loader::utils::cursor::CustomCursor::read_string`].
    #[snafu(display("String Decode Error: `{source}`. (Position: {pos})"))]
    StringDecodeError {
        /// The [`std::string::FromUtf8Error`].
        source: FromUtf8Error,
        /// The start of the C-String attempting to be deserialized.
        pos: u64,
    },

    /// This is a wrapper around [`std::io::Error`] and is used when loading data in [`crate::data_loader::utils::cursor::CustomCursor`]'s methods.
    #[snafu(display("IO Error: `{source}`. (Position: {pos})"))]
    IOError {
        /// The [`std::io::Error`].
        source: io::Error,
        /// The start of the data attempting to be read.
        pos: u64,
    },

    /// Indicates that the cursor escaped the chunk during deserialization.
    /// # Attributes
    #[snafu(display("Escaped the chunk! At position {pos}, should be {loc}."))]
    ChunkEscapeError {
        /// The location of the cursor.
        pos: u64,
        /// The end of the chunk.
        loc: u64,
    },

    /// Indicates that GMSER does not support the version attempting to be deserialized.
    #[snafu(display("GMSER does not currently support version {major}.{minor} (build {build}, release {release}"))]
    InvalidVersionError {
        /// The major version of the file.
        major: u32,
        /// The minor version of the file.
        minor: u32,
        /// The build number of the file.
        build: u32,
        /// The release number of the file.
        release: u32,
    },

    /// Indicates that the program attempted to convert invalid sound flags.
    #[snafu(display("Invalid sound flags: `{flag}`. (Position: {pos})"))]
    InvalidSoundFlags {
        /// The position of the invalid sound flags.
        pos: u64,
        /// The flags in integer representation.
        flag: u32,
    },

    /// Indicates that the program attempted to convert invalid info flags.
    #[snafu(display("Invalid info flags: `{flag}`. (Position: {pos})"))]
    InvalidInfoFlags {
        /// The position of the invalid sound flags.
        pos: u64,
        /// The flags in integer representation.
        flag: u32,
    },

    /// Indicates that GMSER does not support the OPTN chunk version in the file.
    #[snafu(display("Unsupported option version: `{version}`. (Position: {pos})"))]
    UnsupportedOptionVerion {
        /// The position of the invalid option verion.
        pos: u64,
        /// The invalid option version.
        version: u32,
    },

    /// Indicates that the program attempted to convert invalid option flags.
    #[snafu(display("Invalid option flags: `{flag}`. (Position: {pos})"))]
    InvalidOptionFlags {
        /// The position of the invalid sound flags.
        pos: u64,
        /// The flags in integer representation.
        flag: u64,
    },

    /// Indicates that the program attempted to convert invalid.
    /// Notes:
    /// - The classification is a type of bitflag.
    #[snafu(display("Invalid function classifications: `{classifications}`. (Position: {pos})"))]
    InvalidFunctionClassifications {
        /// The position of the invalid sound flags.
        pos: u64,
        /// The classifications in integer representation.
        classifications: u64,
    },

    // Indicates that the program attempted to read a pointer at an invalid location.
    #[snafu(display("Attempting to read a pointer at {pos}."))]
    InvalidReadError {
        /// The location the invalid pointer indicates.
        pos: u64,
    },

    /// Indicates an unsupported texture type header.
    /// # Notes
    /// - Since `[u8; 8]` does not implement [`std::fmt::Display`] it cannot be displayed in the error message.
    #[snafu(display("Invalid texture header at {pos}."))]
    InvalidTextureHeader {
        /// The location the invalid texture type header.
        pos: u64,
        /// The invalid texture texture type header.
        header: [u8; 8],
    },

    /// This is a wrapper around [`std::io::Error`] and indicates there was an issue decompressing a BZip2 texture.
    #[snafu(display("BZip2 Decompression Failure: `{source}`. (Position: {pos})"))]
    BZip2DecompressionError {
        /// The [`std::io::Error`].
        source: io::Error,
        /// The start of the data which is being decompressed.
        pos: u64,
    },

    /// Indicates that there was an invalid texture loading type for a Texture Group Info.
    #[snafu(display("Invalid Texture Load Type: `{load_type}`. (Position: {pos})"))]
    InvalidTextureLoadType {
        /// The texture group info load type.
        load_type: u32,
        /// The position of the load type.
        pos: u64,
    },

    /// Indicates an invalid masking type for a Sprite.
    #[snafu(display("Invalid sprite sep masks type: `{mask_type}`. (Position: {pos})"))]
    InvalidSepMasksType {
        /// The invalid mask type.
        mask_type: u32,
        /// The position of the mask type.
        pos: u64,
    },

    /// Indicates an outdated Sprite type.
    #[snafu(display("Outdated sprite type: `{sprite_type}`. (Position: {pos})"))]
    OutdatedSpriteType {
        /// The sprite type.
        sprite_type: u32,
        /// The location of the sprite type.
        pos: u64,
    },

    /// Indicates an outdated Sprite version.
    /// # Notes
    /// - The sprite version is different from the sprite type
    ///   - The sprite type is `0xFFFFFFFF`, if it is not then it is an old version GMSER does not support.
    ///   - The version is for Game Maker Studio 2 sprites but should be `3` for the versions which GMSER support.
    #[snafu(display("Outdated sprite version: `{sprite_version}`. (Position: {pos})"))]
    OutdatedSpriteVersion {
        /// The sprite version.
        sprite_version: u32,
        /// The location of the sprite version.
        pos: u64,
    },

    /// Indicates that the program attempted to convert an invalid speed type.
    /// Notes:
    /// - The speed type is a type of bitflag.
    #[snafu(display("Invalid playback speed type: `{speed_type}`. (Position: {pos})"))]
    InvalidPlaybackSpeedType {
        /// The invalid speed type in numerical representation.
        speed_type: u32,
        /// The position of the speed type.
        pos: u64,
    },

    /// Indicates that the program attempted to convert an invalid tile mode.
    #[snafu(display("Invalid Tile Mode: `{mode}`. (Position: {pos})"))]
    InvalidTileMode {
        /// The invalid tile mode in numerical representation.
        mode: u32,
        /// The position of the tile mode.
        pos: u64,
    },

    /// This is an error used for debugging.
    /// # Notes
    /// - This is often used for unimplemented sections of code.
    #[snafu(display("{message}. (Position: {pos})"))]
    DebugError {
        /// The debug message to display.
        message: String,
        /// The position of the cursor.
        pos: u64,
    },

    /// Indicates that the program attempted to convert invalid room flags.
    #[snafu(display("Invalid room flags: `{flag}`. (Position: {pos})"))]
    InvalidRoomFlags {
        // The position of the invalid room flags.
        pos: u64,
        // The numerical representation of the room flags.
        flag: u32,
    },

    /// Indicates that the program attempted to convert an invalid layer kind.
    #[snafu(display("Invalid layer kind: `{kind}`. (Position: {pos})"))]
    InvalidLayerKind {
        /// The position of the layer kind.
        pos: u64,
        /// The numerical representation of the layer kind.
        kind: u32,
    },

    /// Invalid program attempted to convert an invalid effect property kind.
    #[snafu(display("Invalid effect property kind: `{kind}`. (Position: {pos})"))]
    InvalidEffectPropertyKind {
        /// The position of the effect property kind.
        pos: u64,
        /// The numerical representation of the effect property kind.
        kind: u32,
    },

    /// Invalid program attempted to convert an invalid collision shape kind.
    #[snafu(display("Invalid collision shape kind: `{kind}`. (Position: {pos})"))]
    InvalidCollisionShapeKind {
        /// The position of the collision shape kind.
        pos: u64,
        /// The numerical representation of the collision shape kind.
        kind: u32,
    },

    /// Indicates that the number of event arrays was different than what was expected.
    /// # Notes
    /// - The event arrays acts as an effective dictionary, as such the number of indices must be controlled.
    #[snafu(display(
        "Invalid event array length. Got `{actual}` expected `{correct}`. (Position: {pos})"
    ))]
    InvalidEventArrayLength {
        /// The start of the event array.
        pos: u64,
        /// The actual size of the event array.
        actual: usize,
        /// The expected size of the event array.
        correct: usize,
    },

    /// Indicates that a given chunk could not be found.
    /// # Notes
    /// - This happens at FORM load time, so there is no cursor position to go off of.
    #[snafu(display("Could not find chunk ident."))]
    MissingChunkIdent {
        /// The identifier of the chunk searched for.
        ident: [u8; 4],
    },
}
