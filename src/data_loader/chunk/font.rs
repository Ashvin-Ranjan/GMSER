use log::info;
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, read_string_callback, CustomCursor},
    error::DataLoadError,
    texture::{deserialize_texture, TextureItem},
};

#[derive(Debug)]
pub struct Kerning {
    pub other: u16,
    pub amount: u16,
}

#[derive(Debug)]
pub struct Glyph {
    pub character: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub shift: u16,
    pub offset: u16,
    pub kerning: Vec<Kerning>,
}

#[derive(Debug)]
pub struct Font {
    pub name: String,
    pub display_name: String,
    pub size: f32,
    pub bold: bool,
    pub italic: bool,
    pub range_start: u16,
    pub charset: u8,
    pub anti_alias: u8,
    pub range_end: u32,
    pub texture_item: TextureItem,
    pub scale_x: f32,
    pub scale_y: f32,
    pub ascender_offset: u32,
    pub ascender: u32,
    pub glyphs: Vec<Glyph>,
}

#[derive(Debug)]
pub struct FontChunk {
    pub size: u32,
    pub fonts: Vec<Font>,
}

impl FontChunk {
    const IDENT: [u8; 4] = [0x46, 0x4F, 0x4E, 0x54]; // "FONT"
}

pub fn deserialize_font(cursor: &mut Cursor<&[u8]>) -> Result<FontChunk, DataLoadError> {
    info!("Deserializing FONT");

    let ident = cursor.read_ident()?;

    if ident != FontChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: FontChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let font_amount = cursor.read_u32()?;
    let mut fonts = Vec::new();
    for _ in 0..font_amount {
        fonts.push(cursor.read_obj_pointer(deserialize_font_obj, 0)?);
    }

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(FontChunk { size, fonts })
}

fn deserialize_font_obj(cursor: &mut Cursor<&[u8]>) -> Result<Font, DataLoadError> {
    let name = cursor.read_obj_pointer(read_string_callback, 0)?;
    let display_name = cursor.read_obj_pointer(read_string_callback, 0)?;
    let size = -cursor.read_f32()?;
    let bold = cursor.read_u32()? != 0;
    let italic = cursor.read_u32()? != 0;
    let range_start = cursor.read_u16()?;
    let charset = cursor.read_u8()?;
    let anti_alias = cursor.read_u8()?;
    let range_end = cursor.read_u32()?;
    let texture_item = cursor.read_obj_pointer(deserialize_texture, 0)?;
    let scale_x = cursor.read_f32()?;
    let scale_y = cursor.read_f32()?;
    let ascender_offset = cursor.read_u32()?;
    let ascender = cursor.read_u32()?;
    let glyphs_count = cursor.read_u32()?;
    let mut glyphs = Vec::new();
    for _ in 0..glyphs_count {
        glyphs.push(deserialize_glyph(cursor)?);
    }

    Ok(Font {
        name,
        display_name,
        size,
        bold,
        italic,
        range_start,
        charset,
        anti_alias,
        range_end,
        texture_item,
        scale_x,
        scale_y,
        ascender_offset,
        ascender,
        glyphs,
    })
}

fn deserialize_glyph(cursor: &mut Cursor<&[u8]>) -> Result<Glyph, DataLoadError> {
    let character = cursor.read_u16()?;
    let x = cursor.read_u16()?;
    let y = cursor.read_u16()?;
    let width = cursor.read_u16()?;
    let height = cursor.read_u16()?;
    let shift = cursor.read_u16()?;
    let offset = cursor.read_u16()?;
    let kerning_amount = cursor.read_u16()?;
    let mut kerning = Vec::new();
    for _ in 0..kerning_amount {
        let other = cursor.read_u16()?;
        let amount = cursor.read_u16()?;
        kerning.push(Kerning { other, amount });
    }

    Ok(Glyph {
        character,
        x,
        y,
        width,
        height,
        shift,
        offset,
        kerning,
    })
}
