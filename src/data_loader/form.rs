//! This is the entry point for deserializing a `*.win` file.
//!
//! # File Format Overview
//! Game Maker Studio data files consist of chunks. These chunks start with a 4-byte identifier
//! and then contain an unsigned 32-bit integer denoting the size of the chunk (excluding the
//! identifier and the size information).
//!
//! Each file starts with a chunk with the identifier `FORM`. The size of the `FORM` chunk is set
//! to be the size of the file minus the 8 bytes for the identifier and the size inofrmation.
//!
//! Each identifer is aligned to 16 bytes, except for `GEN8` (the chunk which starts after `FORM`),
//! which is set at the 8th byte.
//!
//! # Implementation Notes
//! Rather than loading in all of the chunk sequentially, all of the chunk identifers are loaded into
//! a dictionary and then implemented chunks are then loaded. This is to help with future compatibility
//! as there are some chunks which have been inserted into the middle of the file before (`PSEM` and `PSYS`,
//! for example).
//!
//! FormChunk does not implement [`crate::data_loader::utils::cursor::Deserializable`] because it does not allow
//! for outside context, which may be useful in the future.

use log::warn;
use snafu::OptionExt;
use std::{collections::HashMap, io::Cursor};

use crate::data_loader::{
    chunk::{
        agrp::AgrpChunk, audo::AudoChunk, code::CodeChunk, embi::EmbiChunk, feat::FeatChunk,
        font::FontChunk, func::FuncChunk, gen8::Gen8Chunk, glob::GlobChunk, lang::LangChunk,
        objt::ObjtChunk, optn::OptnChunk, path::PathChunk, room::RoomChunk, scpt::ScptChunk,
        sond::SondChunk, sprt::SprtChunk, strg::StrgChunk, tgin::TginChunk, tpag::TpagChunk,
        txtr::TxtrChunk, vari::VariChunk,
    },
    utils::{
        chunk::Chunk,
        cursor::CustomCursor,
        error::{DataLoadError, MissingChunkIdentSnafu},
    },
};

/// This is the main struct which contains all of the deserialized data
/// from the `*.win` file.
pub struct FormChunk {
    /// The size of the file minus 8 bytes for the identifier and size.
    pub size: u32,
    /// The general information chunk.
    pub gen8: Gen8Chunk,
    /// The options chunk.
    pub optn: OptnChunk,
    /// The language groups chunk.
    pub lang: LangChunk,
    /// The sounds chunk.
    pub sond: SondChunk,
    /// The audio groups chunk.
    pub agrp: AgrpChunk,
    /// The sprites chunk.
    pub sprt: SprtChunk,
    /// The paths chunk.
    pub path: PathChunk,
    /// The scripts chunk.
    pub scpt: ScptChunk,
    /// The global variables chunk.
    pub glob: GlobChunk,
    /// The font chunk.
    pub font: FontChunk,
    /// The objects chunk.
    pub objt: ObjtChunk,
    /// The rooms chunk.
    pub room: RoomChunk,
    /// The embedded images chunk.
    pub embi: EmbiChunk,
    /// The texture pages chunk.
    pub tpag: TpagChunk,
    /// The texture group info chunk.
    pub tgin: TginChunk,
    /// The code chunk.
    pub code: CodeChunk,
    /// The variables chunk.
    pub vari: VariChunk,
    /// The functions chunk.
    pub func: FuncChunk,
    /// The features chunk.
    pub feat: FeatChunk,
    /// The strings chunk.
    pub strg: StrgChunk,
    /// The textures chunk.
    pub txtr: TxtrChunk,
    /// The audio chunk.
    pub audo: AudoChunk,
}

impl FormChunk {
    /// Translates to "FORM" (no null terminator).
    const IDENT: [u8; 4] = [0x46, 0x4F, 0x52, 0x4D];
}

/// Locates the start of chunks and places them into a hashmap
///
/// # Arguments
/// - `cursor`: The cursor which contains the file data.
/// - `bounds`: The length of the file.
///
/// # Output
/// Returns a [`HashMap`] where the key is the chunk identifier and the value
/// is the chunk location.
///
/// Returns a [`DataLoadError`] if there was an issue with reading the identifiers.
fn load_chunk_locs(
    cursor: &mut Cursor<&[u8]>,
    bounds: u64,
) -> Result<HashMap<[u8; 4], u64>, DataLoadError> {
    let mut chunk_locs = HashMap::new();
    while cursor.position() < bounds {
        let ident = cursor.read_ident()?;
        let size = cursor.read_u32()?;
        chunk_locs.insert(ident, cursor.position() - 8);
        cursor.set_position(cursor.position() + (size as u64));
    }

    Ok(chunk_locs)
}

/// Deserializes the chunk given the chunk type
///
/// # Example
/// ```
/// let gen8 = load_chunk::<Gen8Chunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
/// ```
/// # Arguments
///
/// - `cursor`: The cursor which contains the file data.
/// - `chunk_locs`: A map which maps chunk identifiers to locations.
/// - `checked_chunks`: A map to edit which maps chunk identifiers to booleans.
///
/// # Output
/// The deserialized chunk.
///
/// It will return a [`DataLoadError`] if there was an issue in deserializing the chunk.
///
/// # Side Effects
/// The value for key `T::IDENT` in `checked_chunks` will be set to `true`.
///
/// # Notes
/// - `checked_chunks` is used as a debugging tool to note any chunks which the program was unable to load.
fn load_chunk<T>(
    cursor: &mut Cursor<&[u8]>,
    chunk_locs: &HashMap<[u8; 4], u64>,
    checked_chunks: &mut HashMap<[u8; 4], bool>,
) -> Result<T, DataLoadError>
where
    T: Chunk,
{
    cursor.set_position(
        *chunk_locs
            .get(&T::IDENT)
            .context(MissingChunkIdentSnafu { ident: T::IDENT })
            .unwrap(),
    );
    checked_chunks.insert(T::IDENT, true);
    T::deserialize(cursor)
}

/// Takes in data as a slice of [`u8`] values and returns a [`FormChunk`].
///
/// # Arguments
/// - `data`: The file as a slice of [`u8`] values.
///
/// # Output
/// The file data deserialized into a [`FormChunk`].
///
/// The function will return an error if there was an issue during deserialization.
pub fn deserialize_form(data: &[u8]) -> Result<FormChunk, DataLoadError> {
    let mut cursor = Cursor::new(data);

    let ident = cursor.read_ident()?;

    if ident != FormChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: FormChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let chunk_locs = load_chunk_locs(&mut cursor, size as u64 + 8)?;
    let mut checked_chunks = HashMap::new();
    for ident in chunk_locs.keys() {
        checked_chunks.insert(*ident, false);
    }

    let gen8 = load_chunk::<Gen8Chunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let optn = load_chunk::<OptnChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let lang = load_chunk::<LangChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let sond = load_chunk::<SondChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let agrp = load_chunk::<AgrpChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let sprt = load_chunk::<SprtChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let path = load_chunk::<PathChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let scpt = load_chunk::<ScptChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let glob = load_chunk::<GlobChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let font = load_chunk::<FontChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let objt = load_chunk::<ObjtChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let room = load_chunk::<RoomChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let embi = load_chunk::<EmbiChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let tpag = load_chunk::<TpagChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let tgin = load_chunk::<TginChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let code = load_chunk::<CodeChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let vari = load_chunk::<VariChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let func = load_chunk::<FuncChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let feat = load_chunk::<FeatChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let strg = load_chunk::<StrgChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let txtr = load_chunk::<TxtrChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let audo = load_chunk::<AudoChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;

    // TODO: As a bit of a check it may be good to add in some warnings here
    // - Check that none of the code locals in the FUNC chunk exceed the limit set in VARI

    for iter in checked_chunks.iter() {
        if !*iter.1 {
            warn!(
                "Did not load chunk {}{}{}{}",
                iter.0[0] as char, iter.0[1] as char, iter.0[2] as char, iter.0[3] as char
            );
        }
    }

    Ok(FormChunk {
        size,
        gen8,
        optn,
        lang,
        sond,
        agrp,
        sprt,
        path,
        scpt,
        glob,
        font,
        objt,
        room,
        embi,
        tpag,
        tgin,
        code,
        vari,
        func,
        feat,
        strg,
        txtr,
        audo,
    })
}
