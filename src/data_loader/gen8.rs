use std::io::{Cursor, Read};

use log::{info, warn};

use snafu::ResultExt;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, read_string_callback, CustomCursor},
    error::{DataLoadError, IOSnafu},
};

#[derive(Debug)]
pub struct VersionInfo {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub release: u32,
    pub format: u8,
}

#[derive(Debug)]
pub struct Gen8Chunk {
    pub size: u32,

    // Data
    pub disable_debug: bool,
    pub version_info: VersionInfo,
    _unk1: u16,
    pub filename: String,
    pub config: String,
    pub last_obj_id: u32,
    pub last_tile_id: u32,
    pub game_id: u32,
    pub legacy_guid: [u8; 16],
    pub game_name: String,
    pub default_window_width: u32,
    pub default_window_height: u32,
    pub info_flags: u32, // TODO: Turn this into an enum type or something
    pub license_crc2: u32,
    pub license_md5: [u8; 16],
    pub timestamp: u64,
    pub display_name: String,
    pub active_targets: u64,
    pub function_classifications: u64, // TODO: Understand this
    pub steam_app_id: u32,
    pub debugger_port: u32,
    pub room_order: Vec<u32>,
    pub random_uid: [u64; 4],
    pub fps: f32,
    pub allow_statistics: bool,
    pub guid: [u8; 16],
}

impl Gen8Chunk {
    const IDENT: [u8; 4] = [0x47, 0x45, 0x4E, 0x38]; // "GEN8"
}

pub fn deserialize_gen8(cursor: &mut Cursor<&[u8]>) -> Result<Gen8Chunk, DataLoadError> {
    info!("Deserializing GEN8");

    let ident = cursor.read_ident()?;

    if ident != Gen8Chunk::IDENT {
        return Result::Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: Gen8Chunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let disable_debug = cursor.read_boolean()?;
    let format_id = cursor.read_u8()?;
    let _unk1 = cursor.read_u16()?;
    let filename = cursor.read_obj_pointer(read_string_callback, 0)?;
    let config = cursor.read_obj_pointer(read_string_callback, 0)?;
    let last_obj_id = cursor.read_u32()?;
    let last_tile_id = cursor.read_u32()?;
    let game_id = cursor.read_u32()?;
    let mut legacy_guid = [0u8; 16];
    cursor
        .read_exact(&mut legacy_guid)
        .context(IOSnafu { pos: start_pos })?;
    let game_name = cursor.read_obj_pointer(read_string_callback, 0)?;

    let major_version = cursor.read_u32()?;
    let minor_version = cursor.read_u32()?;
    let release_number = cursor.read_u32()?;
    let build_number = cursor.read_u32()?;

    if major_version < 2 || format_id < 17 {
        return Err(DataLoadError::InvalidVersionError {
            major: major_version,
            minor: minor_version,
            build: build_number,
            release: release_number,
        });
    }

    let default_window_width = cursor.read_u32()?;
    let default_window_height = cursor.read_u32()?;
    warn!("Still parsing info flags as number. Implement custom parsing");
    let info_flags = cursor.read_u32()?;
    let license_crc2 = cursor.read_u32()?;
    let mut license_md5 = [0u8; 16];
    cursor
        .read_exact(&mut license_md5)
        .context(IOSnafu { pos: start_pos })?;
    let timestamp = cursor.read_u64()?;
    let display_name = cursor.read_obj_pointer(read_string_callback, 0)?;
    let active_targets = cursor.read_u64()?;
    warn!("Still parsing function classifications as number. Implement custom parsing");
    let function_classifications = cursor.read_u64()?;
    let steam_app_id = cursor.read_u32()?;
    let debugger_port = cursor.read_u32()?;

    let room_count = cursor.read_u32()?;
    let mut room_order = Vec::new();
    for _ in 0..room_count {
        room_order.push(cursor.read_u32()?);
    }

    warn!("Missing Random GUID verification from Dog Scepter");
    cursor.read_u64()?; // Throw this away for now
    let mut random_uid = [0u64; 4];
    for i in 0..4 {
        random_uid[i] = cursor.read_u64()?;
    }

    let fps = cursor.read_f32()?;
    let allow_statistics = cursor.read_u32()? != 0;
    let mut game_guid = [0u8; 16];
    cursor
        .read_exact(&mut game_guid)
        .context(IOSnafu { pos: start_pos })?;

    handle_cursor_alignment(cursor, start_pos, size as u64, false)?;

    Ok(Gen8Chunk {
        size,
        disable_debug,
        version_info: VersionInfo {
            major: major_version,
            minor: minor_version,
            build: build_number,
            release: release_number,
            format: format_id,
        },
        _unk1,
        filename,
        config,
        last_obj_id,
        last_tile_id,
        game_id,
        legacy_guid,
        game_name,
        default_window_width,
        default_window_height,
        info_flags,
        license_crc2,
        license_md5,
        timestamp,
        display_name,
        active_targets,
        function_classifications,
        steam_app_id,
        debugger_port,
        room_order,
        random_uid,
        fps,
        allow_statistics,
        guid: game_guid,
    })
}
