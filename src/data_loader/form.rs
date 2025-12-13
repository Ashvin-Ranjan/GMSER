use log::{info, warn};
use std::io::Cursor;

use crate::data_loader::{
    chunk::{
        agrp::AgrpChunk, audo::AudoChunk, code::CodeChunk, embi::EmbiChunk, feat::FeatChunk,
        font::FontChunk, gen8::Gen8Chunk, glob::GlobChunk, lang::LangChunk, objt::ObjtChunk,
        optn::OptnChunk, path::PathChunk, room::RoomChunk, scpt::ScptChunk, sond::SondChunk,
        sprt::SprtChunk, strg::StrgChunk, tgin::TginChunk, tpag::TpagChunk, txtr::TxtrChunk,
    },
    utils::{
        cursor::{CustomCursor, Deserializable},
        error::DataLoadError,
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

    let mut unloaded_data = 0;

    let gen8 = Gen8Chunk::deserialize(&mut cursor)?;
    let optn = OptnChunk::deserialize(&mut cursor)?;
    let lang = LangChunk::deserialize(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // EXTN

    let sond = SondChunk::deserialize(&mut cursor)?;
    let agrp = AgrpChunk::deserialize(&mut cursor)?;
    let sprt = SprtChunk::deserialize(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // BGND

    let path = PathChunk::deserialize(&mut cursor)?;
    let scpt = ScptChunk::deserialize(&mut cursor)?;
    let glob = GlobChunk::deserialize(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // SHDR

    let font = FontChunk::deserialize(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // TMLN

    let objt = ObjtChunk::deserialize(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // FEDS
    unloaded_data += skip_chunk(&mut cursor)?; // ACRV
    unloaded_data += skip_chunk(&mut cursor)?; // SEQN
    unloaded_data += skip_chunk(&mut cursor)?; // TAGS

    let room = RoomChunk::deserialize(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // DAFL

    let embi = EmbiChunk::deserialize(&mut cursor)?;
    let tpag = TpagChunk::deserialize(&mut cursor)?;
    let tgin = TginChunk::deserialize(&mut cursor)?;
    let code = CodeChunk::deserialize(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // VARI
    unloaded_data += skip_chunk(&mut cursor)?; // FUNC

    let feat = FeatChunk::deserialize(&mut cursor)?;
    let strg = StrgChunk::deserialize(&mut cursor)?;
    let txtr = TxtrChunk::deserialize(&mut cursor)?;
    let audo = AudoChunk::deserialize(&mut cursor)?;

    if unloaded_data > 0 {
        warn!(
            "{} bytes ({}%) of file not loaded!",
            unloaded_data,
            (100 * unloaded_data) as f32 / ((size + 8) as f32)
        );
    } else {
        info!("All chunks loaded (you can remove `unloaded_data` now!)");
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

fn skip_chunk(cursor: &mut Cursor<&[u8]>) -> Result<u32, DataLoadError> {
    let ident = cursor.read_ident()?;
    let size = cursor.read_u32()?;
    warn!(
        "Skipping {}{}{}{}. Size: {}",
        ident[0] as char, ident[1] as char, ident[2] as char, ident[3] as char, size,
    );
    cursor.set_position(cursor.position() + (size as u64));
    Ok(size + 8)
}
