use log::{info, warn};
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
    texture::TextureItem,
};

#[derive(Debug)]
pub struct EmbeddedImage {
    pub name: String,
    pub texture: TextureItem,
}

#[derive(Debug)]
pub struct EmbiChunk {
    pub size: u32,
    pub images: Vec<EmbeddedImage>,
}

impl EmbiChunk {
    const IDENT: [u8; 4] = [0x45, 0x4D, 0x42, 0x49]; // "EMBI"
}

impl Deserializable for EmbiChunk {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        info!("Deserializing EMBI");

        let ident = cursor.read_ident()?;

        if ident != EmbiChunk::IDENT {
            return Err(DataLoadError::UnexpectedIdent {
                pos: cursor.position() - 4,
                expected: EmbiChunk::IDENT,
                actual: ident,
            });
        }

        let size = cursor.read_u32()?;

        let start_pos = cursor.position();

        let format_id = cursor.read_u32()?;
        if format_id != 1 {
            warn!("Encountered EMBI format id {}, treating as 1", format_id);
        }

        let image_count = cursor.read_u32()?;
        let mut images = Vec::new();
        for _ in 0..image_count {
            let name = cursor.read_obj_pointer::<String>(0)?;
            let texture = cursor.read_obj_pointer::<TextureItem>(0)?;
            images.push(EmbeddedImage { name, texture })
        }

        handle_cursor_alignment(cursor, start_pos, size as u64, false)?;

        Ok(EmbiChunk { size, images })
    }
}

fn deserialize_embedded_image(cursor: &mut Cursor<&[u8]>) -> Result<EmbeddedImage, DataLoadError> {
    let name = cursor.read_obj_pointer::<String>(0)?;
    let texture = cursor.read_obj_pointer::<TextureItem>(0)?;
    Ok(EmbeddedImage { name, texture })
}
