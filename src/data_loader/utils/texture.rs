//! Contains structs for texture items used in [`crate::data_loader::chunk::font`],
//! [`crate::data_loader::chunk::optn`], [`crate::data_loader::chunk::sprt`],
//! [`crate::data_loader::chunk::embi`], and [`crate::data_loader::chunk::tpag`].

use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{CustomCursor, Deserializable},
    error::DataLoadError,
};

/// This contains general information regarding a texture item on a texture page.
/// # Format Specifications
/// Game Maker Studio stores texture information in texture pages, and then stores texture items to help
/// break apart texture pages.
/// # Notes
/// - This documentation may change in the future as more is found out about Game Maker Studio.
/// - TODO: Many of these values still need to be fully understood.
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
    /// The id of the [`crate::data_loader::chunk::txtr::TexturePage`] this TextureItem belongs to.
    pub texture_page_id: u16,
}

impl Deserializable for TextureItem {
    /// Deserialization for [`TextureItem`].
    /// # Format Specification
    /// Deserialization of [`TextureItem`] is done by reading in all of the
    /// fields as [`u16`]. There are no modifications made to the fields.
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
