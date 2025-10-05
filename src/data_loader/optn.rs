use std::io::Cursor;

use log::{info, warn};

use crate::data_loader::utils::{cursor::CustomCursor, error::DataLoadError};

#[derive(Debug)]
pub struct OptnChunk {
    pub size: u32,
}

impl OptnChunk {
    const IDENT: [u8; 4] = [0x4F, 0x50, 0x54, 0x4E]; // "OPTN"
}

pub fn deserialize_optn(cursor: &mut Cursor<&[u8]>) -> Result<OptnChunk, DataLoadError> {
    info!("Deserializing OPTN");

    let ident = cursor.read_ident()?;

    if ident != OptnChunk::IDENT {
        return Result::Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: OptnChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;
    warn!(
        "OPTN Chunk deserailization not implemented, skipping to {}.",
        cursor.position() + (size as u64)
    );
    cursor.set_position(cursor.position() + (size as u64));

    Ok(OptnChunk { size })
}
