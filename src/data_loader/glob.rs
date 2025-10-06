use log::info;
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct GlobChunk {
    pub size: u32,
    pub globals: Vec<u32>,
}

impl GlobChunk {
    const IDENT: [u8; 4] = [0x47, 0x4C, 0x4F, 0x42]; // "GLOB"
}

pub fn deserialize_glob(cursor: &mut Cursor<&[u8]>) -> Result<GlobChunk, DataLoadError> {
    info!("Deserializing GLOB");

    let ident = cursor.read_ident()?;

    if ident != GlobChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: GlobChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let globals_count = cursor.read_u32()?;
    let mut globals = Vec::new();
    for _ in 0..globals_count {
        globals.push(cursor.read_u32()?);
    }

    handle_cursor_alignment(cursor, start_pos, size as u64, false)?;

    Ok(GlobChunk { size, globals })
}
