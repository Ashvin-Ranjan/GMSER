use log::info;
use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, read_string_callback, CustomCursor},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct StrgChunk {
    pub size: u32,
    pub string_map: HashMap<u32, String>,
}

impl StrgChunk {
    const IDENT: [u8; 4] = [0x53, 0x54, 0x52, 0x47]; // "STRG"
}

pub fn deserialize_strg(cursor: &mut Cursor<&[u8]>) -> Result<StrgChunk, DataLoadError> {
    info!("Deserializing STRG");

    let ident = cursor.read_ident()?;

    if ident != StrgChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: StrgChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let string_map = cursor.read_pointer_map(read_string_callback, 4)?;

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(StrgChunk { size, string_map })
}
