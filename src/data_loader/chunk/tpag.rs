use log::info;
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor},
    error::DataLoadError,
    texture::TextureItem,
};

#[derive(Debug)]
pub struct TpagChunk {
    pub size: u32,
    pub textures: Vec<TextureItem>,
}

impl TpagChunk {
    const IDENT: [u8; 4] = [0x54, 0x50, 0x41, 0x47]; // "TPAG"
}

pub fn deserialize_tpag(cursor: &mut Cursor<&[u8]>) -> Result<TpagChunk, DataLoadError> {
    info!("Deserializing TPAG");

    let ident = cursor.read_ident()?;

    if ident != TpagChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: TpagChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let textures = cursor.read_pointer_list::<TextureItem>(0)?;

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(TpagChunk { size, textures })
}
