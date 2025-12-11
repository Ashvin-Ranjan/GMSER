use log::info;
use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct Script {
    pub name: String,
    pub code_id: u32,
    pub constructor: bool,
}

#[derive(Debug)]
pub struct ScptChunk {
    pub size: u32,
    pub script_map: HashMap<u32, Script>,
}

impl ScptChunk {
    const IDENT: [u8; 4] = [0x53, 0x43, 0x50, 0x54]; // "SCPT"
}

impl Deserializable for ScptChunk {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        info!("Deserializing SCPT");

        let ident = cursor.read_ident()?;

        if ident != ScptChunk::IDENT {
            return Err(DataLoadError::UnexpectedIdent {
                pos: cursor.position() - 4,
                expected: ScptChunk::IDENT,
                actual: ident,
            });
        }

        let size = cursor.read_u32()?;

        let start_pos = cursor.position();

        let script_map = cursor.read_pointer_map::<Script>(0)?;

        handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

        Ok(ScptChunk { size, script_map })
    }
}

impl Deserializable for Script {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let code_id_unmasked = cursor.read_u32()?;
        let constructor = code_id_unmasked >> 31 == 1; // Is constructor if MSB is set
        let code_id = code_id_unmasked & 0x7FFFFFFF;

        Ok(Script {
            name,
            code_id,
            constructor,
        })
    }
}
