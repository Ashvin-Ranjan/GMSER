use log::info;
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct AudioGroup {
    pub name: String,
}

#[derive(Debug)]
pub struct AgrpChunk {
    pub size: u32,
    pub audio_groups: Vec<AudioGroup>,
}

impl AgrpChunk {
    const IDENT: [u8; 4] = [0x41, 0x47, 0x52, 0x50]; // "AGRP"
}

pub fn deserialize_agrp(cursor: &mut Cursor<&[u8]>) -> Result<AgrpChunk, DataLoadError> {
    info!("Deserializing AGRP");

    let ident = cursor.read_ident()?;

    if ident != AgrpChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: AgrpChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let audio_groups = cursor.read_pointer_list::<AudioGroup>(0)?;

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(AgrpChunk { size, audio_groups })
}

impl Deserializable for AudioGroup {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;

        Ok(AudioGroup { name })
    }
}
