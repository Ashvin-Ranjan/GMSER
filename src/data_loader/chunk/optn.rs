use bitflags::bitflags;
use log::info;
use snafu::OptionExt;
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, read_string_callback, CustomCursor},
    error::{DataLoadError, InvalidOptionFlagsSnafu},
    texture::{deserialize_texture, TextureItem},
};

// These flag values are directly taken from DogScepter
bitflags! {
    #[derive(Debug)]
    pub struct OptionFlags: u64 {
        const    FULLSCREEN = 0x1;
        const    INTERPOLATE_PIXELS = 0x2;
        const    USE_NEW_AUDIO = 0x4;
        const    NO_BORDER = 0x8;
        const    SHOW_CURSOR = 0x10;
        const    SIZABLE = 0x20;
        const    STAY_ON_TOP = 0x40;
        const    CHANGE_RESOLUTION = 0x80;
        const    NO_BUTTONS = 0x100;
        const    SCREEN_KEY = 0x200;
        const    HELP_KEY = 0x400;
        const    QUIT_KEY = 0x800;
        const    SAVE_KEY = 0x1000;
        const    SCREENSHOT_KEY = 0x2000;
        const    CLOSE_SEC = 0x4000;
        const    FREEZE = 0x8000;
        const    SHOW_PROGRESS = 0x10000;
        const    LOAD_TRANSPARENT = 0x20000;
        const    SCALE_PROGRESS = 0x40000;
        const    DISPLAY_ERRORS = 0x80000;
        const    WRITE_ERRORS = 0x100000;
        const    ABORT_ERRORS = 0x200000;
        const    VARIABLE_ERRORS = 0x400000;
        const    CREATION_EVENT_ORDER = 0x800000;
        const    USE_FRONT_TOUCH = 0x1000000;
        const    USE_REAR_TOUCH = 0x2000000;
        const    USE_FAST_COLLISION = 0x4000000;
        const    FAST_COLLISION_COMPATIBILITY = 0x8000000;
        const    DISABLE_SANDBOX = 0x10000000;
        const    COPY_ON_WRITE_ENABLED = 0x20000000;
    }
}

#[derive(Debug)]
pub struct Constant {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct OptnChunk {
    pub size: u32,

    // Data
    _unk1: u32,
    option_flags: OptionFlags,
    scale: u32,
    window_color: u32,
    color_depth: u32,
    resolution: u32,
    frequency: u32,
    vertex_sync: u32,
    priority: u32,
    splash_back_image: Option<TextureItem>,
    splash_front_image: Option<TextureItem>,
    splash_load_image: Option<TextureItem>,
    load_alpha: u32,
    constants: Vec<Constant>,
}

impl OptnChunk {
    const IDENT: [u8; 4] = [0x4F, 0x50, 0x54, 0x4E]; // "OPTN"
}

pub fn deserialize_optn(cursor: &mut Cursor<&[u8]>) -> Result<OptnChunk, DataLoadError> {
    info!("Deserializing OPTN");

    let ident = cursor.read_ident()?;

    if ident != OptnChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: OptnChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let option_version = cursor.read_u32()?;

    if option_version != 0x80000000 {
        return Err(DataLoadError::UnsupportedOptionVerion {
            pos: cursor.position() - 4,
            version: option_version,
        });
    }

    let _unk1 = cursor.read_u32()?;

    let option_flags_number = cursor.read_u64()?;
    let option_flags =
        OptionFlags::from_bits(option_flags_number).context(InvalidOptionFlagsSnafu {
            pos: cursor.position() - 8,
            flag: option_flags_number,
        })?;

    let scale = cursor.read_u32()?;
    let window_color = cursor.read_u32()?;
    let color_depth = cursor.read_u32()?;
    let resolution = cursor.read_u32()?;
    let frequency = cursor.read_u32()?;
    let vertex_sync = cursor.read_u32()?;
    let priority = cursor.read_u32()?;

    let splash_back_image = cursor.read_opt_pointer(deserialize_texture, 0)?;
    let splash_front_image = cursor.read_opt_pointer(deserialize_texture, 0)?;
    let splash_load_image = cursor.read_opt_pointer(deserialize_texture, 0)?;
    let load_alpha = cursor.read_u32()?;

    let constants_amount = cursor.read_u32()?;
    let mut constants = Vec::new();
    for _ in 0..constants_amount {
        let name = cursor.read_obj_pointer(read_string_callback, 0)?;
        let value = cursor.read_obj_pointer(read_string_callback, 0)?;
        constants.push(Constant { name, value });
    }

    handle_cursor_alignment(cursor, start_pos, size as u64, false)?;

    Ok(OptnChunk {
        size,
        _unk1,
        option_flags,
        scale,
        window_color,
        color_depth,
        resolution,
        frequency,
        vertex_sync,
        priority,
        splash_back_image,
        splash_front_image,
        splash_load_image,
        load_alpha,
        constants,
    })
}
