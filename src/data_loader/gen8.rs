use bitflags::bitflags;
use log::{info, warn};
use snafu::{OptionExt, ResultExt};
use std::io::{Cursor, Read};

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, read_string_callback, CustomCursor},
    error::{DataLoadError, IOSnafu, InvalidFunctionClassificationsSnafu, InvalidInfoFlagsSnafu},
};

// These flag values are directly taken from DogScepter
bitflags! {
    #[derive(Debug)]
    pub struct InfoFlags: u32 {
        const FULLSCREEN = 0x0001;        // Start fullscreen
        const SYNC_VERTEX1 = 0x0002;       // Use synchronization to avoid tearing
        const SYNC_VERTEX2 = 0x0004;
        const INTERPOLATE = 0x0008;       // Interpolate colours between pixels
        const SCALE = 0x0010;             // Scaling: Keep aspect
        const SHOW_CURSOR = 0x0020;        // Display cursor
        const SIZABLE = 0x0040;          // Allow window resize
        const SCREEN_KEY = 0x0080;         // Allow fullscreen switching
        const SYNC_VERTEX3 = 0x0100;
        const STUDIO_VERSION_B1 = 0x0200;
        const STUDIO_VERSION_B2 = 0x0400;
        const STUDIO_VERSION_B3 = 0x0800;
        const STUDIO_VERSION_MASK = 0x0E00; // studioVersion = (infoFlags & InfoFlags.StudioVersionMask) >> 9
        const STEAM_OR_PLAYER = 0x1000;     // Steam or YoYo Player
        const LOCAL_DATA_ENABLED = 0x2000;
        const BORDERLESS_WINDOW = 0x4000;  // Borderless Window
        const DEFAULT_CODE_KIND = 0x8000;
        const LICENSE_EXCLUSIONS = 0x10000;
    }
    #[derive(Debug)]
    pub struct FunctionClassifications: u64 {
        const INTERNET = 0x1;
        const JOYSTICK = 0x2;
        const GAMEPAD = 0x4;
        const READ_SCREEN_PIXELS = 0x10;
        const MATH = 0x20;
        const ACTION = 0x40;
        const D3D_STATE = 0x80;
        const D3D_PRIMITIVE = 0x100;
        const DATA_STRUCTURE = 0x200;
        const FILE_LEGACY = 0x400;
        const INI = 0x800;
        const FILENAME = 0x1000;
        const DIRECTORY = 0x2000;
        const SHELL = 0x4000;
        const OBSOLETE = 0x8000;
        const HTTP = 0x10000;
        const JSON_ZIP = 0x20000;
        const DEBUG = 0x40000;
        const MOTION = 0x80000;
        const COLLISION = 0x100000;
        const INSTANCE = 0x200000;
        const ROOM = 0x400000;
        const GAME = 0x800000;
        const DISPLAY = 0x1000000;
        const DEVICE = 0x2000000;
        const WINDOW = 0x4000000;
        const DRAW = 0x8000000;
        const TEXTURE = 0x10000000;
        const GRAPHICS = 0x20000000;
        const STRING = 0x40000000;
        const TILE = 0x80000000;
        const SURFACE = 0x100000000;
        const SKELETON = 0x200000000;
        const IO = 0x400000000;
        const GM_SYSTEM = 0x800000000;
        const ARRAY = 0x1000000000;
        const EXTERNAL = 0x2000000000;
        const PUSH = 0x4000000000;
        const DATE = 0x8000000000;
        const PARTICLE = 0x10000000000;
        const RESOURCE = 0x20000000000;
        const HTML5 = 0x40000000000;
        const SOUND = 0x80000000000;
        const AUDIO = 0x100000000000;
        const EVENT = 0x200000000000;
        const SCRIPT = 0x400000000000;
        const TEXT = 0x800000000000;
        const ANALYTICS = 0x1000000000000;
        const OBJECT = 0x2000000000000;
        const ASSET = 0x4000000000000;
        const ACHIEVEMENT = 0x8000000000000;
        const CLOUD = 0x10000000000000;
        const ADS = 0x20000000000000;
        const OS = 0x40000000000000;
        const IAP = 0x80000000000000;
        const FACEBOOK = 0x100000000000000;
        const PHYSICS = 0x200000000000000;
        const SWF = 0x400000000000000;
        const PLATFORM_SPECIFIC = 0x800000000000000;
        const BUFFER = 0x1000000000000000;
        const STEAM = 0x2000000000000000;
        const STEAM_UGC = 0x2010000000000000;
        const SHADER = 0x4000000000000000;
        const VERTEX = 0x8000000000000000;
    }
}

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
    pub info_flags: InfoFlags,
    pub license_crc2: u32,
    pub license_md5: [u8; 16],
    pub timestamp: u64,
    pub display_name: String,
    pub active_targets: u64,
    pub function_classifications: FunctionClassifications,
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
        return Err(DataLoadError::UnexpectedIdent {
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

    let info_flags_number = cursor.read_u32()?;
    let info_flags = InfoFlags::from_bits(info_flags_number).context(InvalidInfoFlagsSnafu {
        pos: cursor.position() - 4,
        flag: info_flags_number,
    })?;

    let license_crc2 = cursor.read_u32()?;
    let mut license_md5 = [0u8; 16];
    cursor
        .read_exact(&mut license_md5)
        .context(IOSnafu { pos: start_pos })?;
    let timestamp = cursor.read_u64()?;
    let display_name = cursor.read_obj_pointer(read_string_callback, 0)?;
    let active_targets = cursor.read_u64()?;

    let function_classifications_number = cursor.read_u64()?;
    let function_classifications = FunctionClassifications::from_bits(
        function_classifications_number,
    )
    .context(InvalidFunctionClassificationsSnafu {
        pos: cursor.position() - 8,
        classifications: function_classifications_number,
    })?;

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
