use log::{info, warn};
use snafu::OptionExt;
use std::{collections::HashMap, io::Cursor};

use crate::data_loader::{
    chunk::{
        agrp::AgrpChunk, audo::AudoChunk, code::CodeChunk, embi::EmbiChunk, feat::FeatChunk,
        font::FontChunk, gen8::Gen8Chunk, glob::GlobChunk, lang::LangChunk, objt::ObjtChunk,
        optn::OptnChunk, path::PathChunk, room::RoomChunk, scpt::ScptChunk, sond::SondChunk,
        sprt::SprtChunk, strg::StrgChunk, tgin::TginChunk, tpag::TpagChunk, txtr::TxtrChunk,
    },
    utils::{
        chunk::Chunk,
        cursor::{CustomCursor, Deserializable},
        error::{DataLoadError, MissingChunkIdentSnafu},
    },
};

pub struct FormChunk {
    pub size: u32,
    pub gen8: Gen8Chunk,
    pub optn: OptnChunk,
    pub lang: LangChunk,
    pub sond: SondChunk,
    pub agrp: AgrpChunk,
    pub sprt: SprtChunk,
    pub path: PathChunk,
    pub scpt: ScptChunk,
    pub glob: GlobChunk,
    pub font: FontChunk,
    pub objt: ObjtChunk,
    pub room: RoomChunk,
    pub embi: EmbiChunk,
    pub tpag: TpagChunk,
    pub tgin: TginChunk,
    pub code: CodeChunk,
    pub feat: FeatChunk,
    pub strg: StrgChunk,
    pub txtr: TxtrChunk,
    pub audo: AudoChunk,
}

impl FormChunk {
    const IDENT: [u8; 4] = [0x46, 0x4F, 0x52, 0x4D]; // "FORM"
}

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
    let feat = load_chunk::<FeatChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let strg = load_chunk::<StrgChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let txtr = load_chunk::<TxtrChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;
    let audo = load_chunk::<AudoChunk>(&mut cursor, &chunk_locs, &mut checked_chunks)?;

    for iter in checked_chunks.iter() {
        if *iter.1 {
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
        feat,
        strg,
        txtr,
        audo,
    })
}
