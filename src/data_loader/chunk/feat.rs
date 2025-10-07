use log::info;
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, read_string_callback, CustomCursor},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct FeatChunk {
    pub size: u32,
    pub features: Vec<String>,
}

impl FeatChunk {
    const IDENT: [u8; 4] = [0x46, 0x45, 0x41, 0x54]; // "FEAT"
}

pub fn deserialize_feat(cursor: &mut Cursor<&[u8]>) -> Result<FeatChunk, DataLoadError> {
    info!("Deserializing FEAT");

    let ident = cursor.read_ident()?;

    if ident != FeatChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: FeatChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let features = cursor.read_pointer_list(read_string_callback, 0)?;

    handle_cursor_alignment(cursor, start_pos, size as u64, false)?;

    Ok(FeatChunk { size, features })
}
