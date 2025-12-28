use log::info;
use std::io::Cursor;

use crate::data_loader::utils::{
    chunk::Chunk,
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct Language {
    pub name: String,
    pub region: String,
    pub entries: Vec<String>,
}

#[derive(Debug)]
pub struct LangChunk {
    pub size: u32,
    _unk1: u32,
    pub languages: Vec<Language>,
    pub entry_ids: Vec<String>,
}

impl Chunk for LangChunk {
    const IDENT: [u8; 4] = [0x4C, 0x41, 0x4E, 0x47]; // "LANG"
}

impl Deserializable for LangChunk {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        info!("Deserializing LANG");

        let ident = cursor.read_ident()?;

        if ident != LangChunk::IDENT {
            return Err(DataLoadError::UnexpectedIdent {
                pos: cursor.position() - 4,
                expected: LangChunk::IDENT,
                actual: ident,
            });
        }

        let size = cursor.read_u32()?;

        let start_pos = cursor.position();

        let _unk1 = cursor.read_u32()?;
        let language_count = cursor.read_u32()?;
        let entry_count = cursor.read_u32()?;

        let entry_ids = cursor.read_pointer_list::<String>(0)?;

        let mut languages = Vec::new();
        for _ in 0..language_count {
            languages.push(deserialize_language(cursor, entry_count)?);
        }

        handle_cursor_alignment(cursor, start_pos, size as u64, false)?;

        Ok(LangChunk {
            size,
            _unk1,
            languages,
            entry_ids,
        })
    }
}

fn deserialize_language(
    cursor: &mut Cursor<&[u8]>,
    entry_count: u32,
) -> Result<Language, DataLoadError> {
    let name = cursor.read_obj_pointer::<String>(0)?;
    let region = cursor.read_obj_pointer::<String>(0)?;

    let mut entries = Vec::new();
    for _ in 0..entry_count {
        entries.push(cursor.read_obj_pointer::<String>(0)?);
    }

    Ok(Language {
        name,
        region,
        entries,
    })
}
