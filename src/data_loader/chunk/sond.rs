use bitflags::bitflags;
use log::info;
use snafu::OptionExt;
use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, read_string_callback, CustomCursor},
    error::{DataLoadError, InvalidSoundFlagsSnafu},
};

bitflags! {
    #[derive(Debug)]
    pub struct SoundFlags: u32 {
        const IS_EMBEDDED    = 0x1;
        const IS_COMPRESSED    = 0x2;
        const REGULAR   = 0x64;
    }
}

#[derive(Debug)]
pub struct Sound {
    pub name: String,
    pub flag: SoundFlags,
    pub sound_type: Option<String>,
    pub file: String,
    pub effects: u32,
    pub volume: f32,
    pub pitch: f32,
    pub group_id: u32,
    pub audio_id: u32,
}

#[derive(Debug)]
pub struct SondChunk {
    pub size: u32,
    pub sound_map: HashMap<u32, Sound>,
}

impl SondChunk {
    const IDENT: [u8; 4] = [0x53, 0x4F, 0x4E, 0x44]; // "SOND"
}

pub fn deserialize_sond(cursor: &mut Cursor<&[u8]>) -> Result<SondChunk, DataLoadError> {
    info!("Deserializing SOND");

    let ident = cursor.read_ident()?;

    if ident != SondChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: SondChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let sound_map = cursor.read_pointer_map(deserialize_sound, 0)?;

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(SondChunk { size, sound_map })
}

fn deserialize_sound(cursor: &mut Cursor<&[u8]>) -> Result<Sound, DataLoadError> {
    let name = cursor.read_obj_pointer(read_string_callback, 0)?;
    let flag_number = cursor.read_u32()?;
    let flag = SoundFlags::from_bits(flag_number).context(InvalidSoundFlagsSnafu {
        pos: cursor.position() - 4,
        flag: flag_number,
    })?;
    let sound_type = cursor.read_opt_pointer(read_string_callback, 0)?;
    let file = cursor.read_obj_pointer(read_string_callback, 0)?;
    let effects = cursor.read_u32()?;
    let volume = cursor.read_f32()?;
    let pitch = cursor.read_f32()?;
    let group_id = cursor.read_u32()?;
    let audio_id = cursor.read_u32()?;

    Ok(Sound {
        name,
        flag,
        sound_type,
        file,
        effects,
        volume,
        pitch,
        group_id,
        audio_id,
    })
}
