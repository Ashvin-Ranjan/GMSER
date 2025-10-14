use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{CustomCursor, Deserializable},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct TextureItem {
    pub source_x: u16,
    pub source_y: u16,
    pub source_width: u16,
    pub source_height: u16,
    pub target_x: u16,
    pub target_y: u16,
    pub target_width: u16,
    pub target_height: u16,
    pub bound_width: u16,
    pub bound_height: u16,
    pub texture_page_id: u16,
}

impl Deserializable for TextureItem {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let source_x = cursor.read_u16()?;
        let source_y = cursor.read_u16()?;
        let source_width = cursor.read_u16()?;
        let source_height = cursor.read_u16()?;
        let target_x = cursor.read_u16()?;
        let target_y = cursor.read_u16()?;
        let target_width = cursor.read_u16()?;
        let target_height = cursor.read_u16()?;
        let bound_width = cursor.read_u16()?;
        let bound_height = cursor.read_u16()?;
        let texture_page_id = cursor.read_u16()?;

        Ok(TextureItem {
            source_x,
            source_y,
            source_width,
            source_height,
            target_x,
            target_y,
            target_width,
            target_height,
            bound_width,
            bound_height,
            texture_page_id,
        })
    }
}
