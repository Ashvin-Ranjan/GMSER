use bzip2::read::BzDecoder;
use log::info;
use snafu::ResultExt;
use std::io::{Cursor, Read};

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor},
    error::{BZip2DecompressionSnafu, DataLoadError, IOSnafu},
};

#[derive(Debug)]
pub enum TextureType {
    QOIandBZip2,
    QOI,
    PNG,
}

#[derive(Debug)]
pub struct TextureData {
    pub texture_type: TextureType,
    pub data: Vec<u8>,
    pub qoi_length: Option<u32>,
    pub qoi_height: Option<u16>,
    pub qoi_width: Option<u16>,
}

#[derive(Debug)]
pub struct TexturePage {
    pub scaled: u32,
    pub generated_mips: u32,
    pub texture_width: u32,
    pub texture_height: u32,
    pub texture_group: u32,
    pub texture: TextureData,
}

#[derive(Debug)]
pub struct TxtrChunk {
    pub size: u32,
    pub page_list: Vec<TexturePage>,
}

impl TxtrChunk {
    const IDENT: [u8; 4] = [0x54, 0x58, 0x54, 0x52]; // "TXTR"
}

// Taken from DogScepter
impl TextureType {
    const QOI_AND_BZIP2_HEADER: [u8; 4] = [0x32, 0x7A, 0x6F, 0x71];
    const QOI_HEADER: [u8; 4] = [0x66, 0x69, 0x6F, 0x71];
    const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x41, 0xD, 0x0A, 0x1A, 0x0A];
}

pub fn deserialize_txtr(cursor: &mut Cursor<&[u8]>) -> Result<TxtrChunk, DataLoadError> {
    info!("Deserializing TXTR");

    let ident = cursor.read_ident()?;

    if ident != TxtrChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: TxtrChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let page_amount = cursor.read_u32()?;
    let mut page_list = Vec::new();
    for _ in 0..page_amount {
        page_list.push(cursor.read_obj_pointer(deserialize_texture_page, 0)?);
    }

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(TxtrChunk { size, page_list })
}

fn deserialize_texture_page(cursor: &mut Cursor<&[u8]>) -> Result<TexturePage, DataLoadError> {
    let scaled = cursor.read_u32()?;
    let generated_mips = cursor.read_u32()?;
    cursor.read_u32()?; // Discard size
    let texture_width = cursor.read_u32()?;
    let texture_height = cursor.read_u32()?;
    let texture_group = cursor.read_u32()?;
    let texture = cursor.read_obj_pointer(deserialize_texture_item, 0)?;
    return Ok(TexturePage {
        scaled,
        generated_mips,
        texture_width,
        texture_height,
        texture_group,
        texture,
    });
}

fn deserialize_texture_item(cursor: &mut Cursor<&[u8]>) -> Result<TextureData, DataLoadError> {
    let start_pos = cursor.position();
    let mut header = [0u8; 8];
    cursor
        .read_exact(&mut header)
        .context(IOSnafu { pos: start_pos })?;

    if header[0..4] == TextureType::QOI_AND_BZIP2_HEADER {
        cursor.set_position(cursor.position() - 4);
        let qoi_width = Some(cursor.read_u16()?);
        let qoi_height = Some(cursor.read_u16()?);
        let qoi_length = Some(cursor.read_u32()?);
        let mut decoder = BzDecoder::new(cursor);

        // Decompress
        let mut data = Vec::new();
        decoder
            .read_to_end(&mut data)
            .context(BZip2DecompressionSnafu {
                pos: start_pos + 12,
            })?;

        return Ok(TextureData {
            texture_type: TextureType::QOIandBZip2,
            data,
            qoi_length,
            qoi_height,
            qoi_width,
        });
    } else if header[0..4] == TextureType::QOI_HEADER {
        cursor.set_position(cursor.position() - 4);
        let qoi_width = Some(cursor.read_u16()?);
        let qoi_height = Some(cursor.read_u16()?);
        let qoi_length = cursor.read_u32()?;
        cursor.set_position(cursor.position() - 12);
        let mut data = vec![0u8; qoi_length as usize + 12];
        cursor
            .read_exact(&mut data)
            .context(IOSnafu { pos: start_pos + 8 })?;
        return Ok(TextureData {
            texture_type: TextureType::QOI,
            data,
            qoi_length: Some(qoi_length),
            qoi_height,
            qoi_width,
        });
    } else if header == TextureType::PNG_HEADER {
        let mut chunk_type = 0;
        while chunk_type != 0x444E4549
        /* IEND */
        {
            let length = cursor.read_u32()?;
            chunk_type = cursor.read_u32()?;
            cursor.set_position(cursor.position() + (length as u64) + 4);
        }

        let data_len = cursor.position() - (start_pos + 8);
        cursor.set_position(start_pos + 8);
        let mut data = vec![0u8; data_len as usize];
        cursor
            .read_exact(&mut data)
            .context(IOSnafu { pos: start_pos + 8 })?;
        return Ok(TextureData {
            texture_type: TextureType::PNG,
            data,
            qoi_length: None,
            qoi_height: None,
            qoi_width: None,
        });
    }

    Err(DataLoadError::InvalidTextureHeader {
        pos: start_pos,
        header,
    })
}
