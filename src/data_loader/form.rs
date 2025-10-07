use log::{info, warn};
use std::io::Cursor;

use crate::data_loader::{
    chunk::{
        agrp::{deserialize_agrp, AgrpChunk},
        audo::{deserialize_audo, AudoChunk},
        embi::{deserialize_embi, EmbiChunk},
        feat::{deserialize_feat, FeatChunk},
        font::{deserialize_font, FontChunk},
        gen8::{deserialize_gen8, Gen8Chunk},
        glob::{deserialize_glob, GlobChunk},
        lang::{deserialize_lang, LangChunk},
        optn::{deserialize_optn, OptnChunk},
        path::{deserialize_path, PathChunk},
        scpt::{deserialize_scpt, ScptChunk},
        sond::{deserialize_sond, SondChunk},
        strg::{deserialize_strg, StrgChunk},
        tgin::{deserialize_tgin, TginChunk},
        tpag::{deserialize_tpag, TpagChunk},
        txtr::{deserialize_txtr, TxtrChunk},
    },
    utils::{cursor::CustomCursor, error::DataLoadError},
};

pub struct FormChunk {
    pub size: u32,
    pub gen8: Gen8Chunk,
    pub optn: OptnChunk,
    pub lang: LangChunk,
    pub sond: SondChunk,
    pub agrp: AgrpChunk,
    pub path: PathChunk,
    pub scpt: ScptChunk,
    pub glob: GlobChunk,
    pub font: FontChunk,
    pub embi: EmbiChunk,
    pub tpag: TpagChunk,
    pub tgin: TginChunk,
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

    let gen8 = deserialize_gen8(&mut cursor)?;
    let optn = deserialize_optn(&mut cursor)?;
    let lang = deserialize_lang(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // EXTN

    let sond = deserialize_sond(&mut cursor)?;
    let agrp = deserialize_agrp(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // SPRT
    unloaded_data += skip_chunk(&mut cursor)?; // BGND

    let path = deserialize_path(&mut cursor)?;
    let scpt = deserialize_scpt(&mut cursor)?;
    let glob = deserialize_glob(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // SHDR

    let font = deserialize_font(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // TMLN
    unloaded_data += skip_chunk(&mut cursor)?; // OBJT
    unloaded_data += skip_chunk(&mut cursor)?; // FEDS
    unloaded_data += skip_chunk(&mut cursor)?; // ACRV
    unloaded_data += skip_chunk(&mut cursor)?; // SEQN
    unloaded_data += skip_chunk(&mut cursor)?; // TAGS
    unloaded_data += skip_chunk(&mut cursor)?; // ROOM
    unloaded_data += skip_chunk(&mut cursor)?; // DAFL

    let embi = deserialize_embi(&mut cursor)?;
    let tpag = deserialize_tpag(&mut cursor)?;
    let tgin = deserialize_tgin(&mut cursor)?;

    unloaded_data += skip_chunk(&mut cursor)?; // CODE
    unloaded_data += skip_chunk(&mut cursor)?; // VARI
    unloaded_data += skip_chunk(&mut cursor)?; // FUNC

    let feat = deserialize_feat(&mut cursor)?;
    let strg = deserialize_strg(&mut cursor)?;
    let txtr = deserialize_txtr(&mut cursor)?;
    let audo = deserialize_audo(&mut cursor)?;

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
        path,
        scpt,
        glob,
        font,
        embi,
        tpag,
        tgin,
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
