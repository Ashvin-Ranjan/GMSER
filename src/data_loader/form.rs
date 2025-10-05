use log::warn;
use std::io::Cursor;

use crate::data_loader::{
    audo::{deserialize_audo, AudoChunk},
    gen8::{deserialize_gen8, Gen8Chunk},
    lang::{deserialize_lang, LangChunk},
    optn::{deserialize_optn, OptnChunk},
    sond::{deserialize_sond, SondChunk},
    strg::{deserialize_strg, StrgChunk},
    utils::{cursor::CustomCursor, error::DataLoadError},
};

pub struct FormChunk {
    pub size: u32,
    pub gen8: Gen8Chunk,
    pub optn: OptnChunk,
    pub lang: LangChunk,
    pub strg: StrgChunk,
    pub sond: SondChunk,
    pub audo: AudoChunk,
}

impl FormChunk {
    const IDENT: [u8; 4] = [0x46, 0x4F, 0x52, 0x4D]; // "FORM"
}

pub fn deserialize_form(data: &[u8]) -> Result<FormChunk, DataLoadError> {
    let mut cursor = Cursor::new(data);

    let ident = cursor.read_ident()?;

    if ident != FormChunk::IDENT {
        return Result::Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: FormChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let gen8 = deserialize_gen8(&mut cursor)?;
    let optn = deserialize_optn(&mut cursor)?;
    let lang = deserialize_lang(&mut cursor)?;

    skip_chunk(&mut cursor)?; // EXTN

    let sond = deserialize_sond(&mut cursor)?;

    skip_chunk(&mut cursor)?; // AGRP
    skip_chunk(&mut cursor)?; // SPRT
    skip_chunk(&mut cursor)?; // BGND
    skip_chunk(&mut cursor)?; // PATH
    skip_chunk(&mut cursor)?; // SCPT
    skip_chunk(&mut cursor)?; // GLOB
    skip_chunk(&mut cursor)?; // SHDR
    skip_chunk(&mut cursor)?; // FONT
    skip_chunk(&mut cursor)?; // TMLN
    skip_chunk(&mut cursor)?; // OBJT
    skip_chunk(&mut cursor)?; // FEDS
    skip_chunk(&mut cursor)?; // ACRV
    skip_chunk(&mut cursor)?; // SEQN
    skip_chunk(&mut cursor)?; // TAGS
    skip_chunk(&mut cursor)?; // ROOM
    skip_chunk(&mut cursor)?; // DAFL
    skip_chunk(&mut cursor)?; // EMBI
    skip_chunk(&mut cursor)?; // TPAG
    skip_chunk(&mut cursor)?; // TGIN
    skip_chunk(&mut cursor)?; // CODE
    skip_chunk(&mut cursor)?; // VARI
    skip_chunk(&mut cursor)?; // FUNC
    skip_chunk(&mut cursor)?; // FEAT

    let strg = deserialize_strg(&mut cursor)?;

    skip_chunk(&mut cursor)?; // TXTR

    let audo = deserialize_audo(&mut cursor)?;

    Ok(FormChunk {
        size,
        gen8,
        optn,
        lang,
        strg,
        sond,
        audo,
    })
}

fn skip_chunk(cursor: &mut Cursor<&[u8]>) -> Result<(), DataLoadError> {
    let ident = cursor.read_ident()?;
    warn!(
        "Skipping {}{}{}{}",
        ident[0] as char, ident[1] as char, ident[2] as char, ident[3] as char
    );
    let size = cursor.read_u32()?;
    cursor.set_position(cursor.position() + (size as u64));
    Ok(())
}
