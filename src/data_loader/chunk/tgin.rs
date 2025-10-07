use log::{info, warn};
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, read_string_callback, CustomCursor},
    error::DataLoadError,
};

#[derive(Debug)]
pub enum LoadType {
    InFile,
    SeparateGroup,
    SeparateTextures,
}

#[derive(Debug)]
pub struct TextureGroupInfo {
    pub name: String,
    pub directory: String,
    pub extension: String,
    pub load_type: LoadType,
    pub texture_page_ids: Vec<u32>,
    pub sprite_ids: Vec<u32>,
    pub font_ids: Vec<u32>,
    pub tileset_ids: Vec<u32>,
}

#[derive(Debug)]
pub struct TginChunk {
    pub size: u32,
    pub texture_groups: Vec<TextureGroupInfo>,
}

impl TginChunk {
    const IDENT: [u8; 4] = [0x54, 0x47, 0x49, 0x4E]; // "TGIN"
}

pub fn deserialize_tgin(cursor: &mut Cursor<&[u8]>) -> Result<TginChunk, DataLoadError> {
    info!("Deserializing TGIN");

    let ident = cursor.read_ident()?;

    if ident != TginChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: TginChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let format_id = cursor.read_u32()?;
    if format_id != 1 {
        warn!("Encountered TGIN format id {}, treating as 1", format_id);
    }

    let texture_groups = cursor.read_pointer_list(deserialize_texture_group_info, 0)?;

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(TginChunk {
        size,
        texture_groups,
    })
}

fn deserialize_texture_group_info(
    cursor: &mut Cursor<&[u8]>,
) -> Result<TextureGroupInfo, DataLoadError> {
    let name = cursor.read_obj_pointer(read_string_callback, 0)?;
    let directory = cursor.read_obj_pointer(read_string_callback, 0)?;
    let extension = cursor.read_obj_pointer(read_string_callback, 0)?;
    let attempted_load_type = cursor.read_u32()?;
    let load_type = match attempted_load_type {
        0 => Ok(LoadType::InFile),
        1 => Ok(LoadType::SeparateGroup),
        2 => Ok(LoadType::SeparateTextures),
        _ => Err(DataLoadError::InvalidTextureLoadType {
            load_type: attempted_load_type,
            pos: cursor.position() - 4,
        }),
    }?;
    let texture_page_ids = cursor.read_obj_pointer(deserialize_resource_ids, 0)?;
    let sprite_ids = cursor.read_obj_pointer(deserialize_resource_ids, 0)?;
    let font_ids = cursor.read_obj_pointer(deserialize_resource_ids, 0)?;
    let tileset_ids = cursor.read_obj_pointer(deserialize_resource_ids, 0)?;

    Ok(TextureGroupInfo {
        name,
        directory,
        extension,
        load_type,
        texture_page_ids,
        sprite_ids,
        font_ids,
        tileset_ids,
    })
}

fn deserialize_resource_ids(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u32>, DataLoadError> {
    let amount = cursor.read_u32()?;
    let mut output = Vec::new();
    for _ in 0..amount {
        output.push(cursor.read_u32()?);
    }
    Ok(output)
}
