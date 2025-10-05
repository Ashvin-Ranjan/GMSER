use std::io::Cursor;

use log::info;

use crate::data_loader::{
    gen8::{deserialize_gen8, Gen8Chunk},
    utils::{cursor::CustomCursor, error::DataLoadError},
};

pub struct FormChunk {
    pub size: u32,
    pub gen8: Gen8Chunk,
}

impl FormChunk {
    const IDENT: [u8; 4] = [0x46, 0x4F, 0x52, 0x4D]; // "FORM"
}

pub fn deserialize_form(data: &[u8]) -> Result<FormChunk, DataLoadError> {
    let mut cursor = Cursor::new(data);

    let ident = cursor.read_ident()?;

    if ident != FormChunk::IDENT {
        return Result::Err(DataLoadError::UnexpectedIdent {
            pos: 0,
            expected: FormChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    info!("Parsing GEN8");

    let gen8 = deserialize_gen8(&mut cursor)?;

    Ok(FormChunk {
        size: size,
        gen8: gen8,
    })
}
