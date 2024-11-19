use crate::{read_adv, FILE_DATA};

use super::chunk::Chunk;

pub struct GameInfo {
    pub room_order: Vec<u32>,
    pub legacy_uuid: u128,
    pub game_uuid: u128,
    pub active_targets: u64,
    pub function_classifications: u64,
    pub timestamp: u64,
    pub filename_pointer: u32,
    pub config_pointer: u32,
    pub last_object_id: u32,
    pub last_tile_id: u32,
    pub game_id: u32,
    pub game_name_pointer: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub release: u32,
    pub build: u32,
    pub default_window_width: u32,
    pub default_window_height: u32,
    pub info_flags: u32,
    pub display_name_pointer: u32,
    pub room_count: u32,
    pub debug_port: u32,
    pub unknown: u16 , // TODO: Figure out what this does
    pub fps: f32,
    pub allow_statistics: bool,
    pub disable_debug: bool,
    pub format_id: u8,
}

fn default_game_info() -> GameInfo {
    GameInfo {
        room_order: Vec::new(),
        legacy_uuid: 0,
        game_uuid: 0,
        active_targets: 0,
        function_classifications: 0,
        timestamp: 0,
        filename_pointer: 0,
        config_pointer: 0,
        last_object_id: 0,
        last_tile_id: 0,
        game_id: 0,
        game_name_pointer: 0,
        major_version: 0,
        minor_version: 0,
        release: 0,
        build: 0,
        default_window_width: 0,
        default_window_height: 0,
        info_flags: 0,
        display_name_pointer: 0,
        room_count: 0,
        debug_port: 0,
        unknown: 0 , // TODO: Figure out what this does
        fps: 0.0,
        allow_statistics: false,
        disable_debug: false,
        format_id: 0,
    }
}

pub fn read_game_info(chunk: Chunk) -> GameInfo {
    let mut game_info: GameInfo = default_game_info();

    let slice = FILE_DATA.as_slice();
    let mut pointer = chunk.start as usize;
    
    game_info.disable_debug = read_adv!(u8, slice, pointer) == 1;
    game_info.format_id = read_adv!(u8, slice, pointer);
    game_info.unknown = read_adv!(u16, slice, pointer);
    game_info.filename_pointer = read_adv!(u32, slice, pointer);
    game_info.config_pointer = read_adv!(u32, slice, pointer);
    game_info.last_object_id = read_adv!(u32, slice, pointer); 
    game_info.last_tile_id = read_adv!(u32, slice, pointer);
    game_info.game_id = read_adv!(u32, slice, pointer);
    game_info.legacy_uuid = read_adv!(u128, slice, pointer);
    game_info.game_name_pointer = read_adv!(u32, slice, pointer);
    game_info.major_version = read_adv!(u32, slice, pointer);
    game_info.minor_version = read_adv!(u32, slice, pointer);
    game_info.release = read_adv!(u32, slice, pointer);
    game_info.build = read_adv!(u32, slice, pointer);
    game_info.default_window_width = read_adv!(u32, slice, pointer);
    game_info.default_window_height = read_adv!(u32, slice, pointer);
    game_info.info_flags = read_adv!(u32, slice, pointer);
    pointer += 20; // TODO: Handle licenses and all that
    game_info.timestamp = read_adv!(u64, slice, pointer);
    game_info.display_name_pointer = read_adv!(u32, slice, pointer);
    game_info.active_targets = read_adv!(u64, slice, pointer);
    game_info.function_classifications = read_adv!(u64, slice, pointer); // TODO: Handle these
    pointer += 4; // TODO: Handle Steam ID
    if game_info.format_id >= 14 {
        game_info.debug_port = read_adv!(u32, slice, pointer);
    }
    game_info.room_count = read_adv!(u32, slice, pointer);
    for _ in 0..game_info.room_count {
        game_info.room_order.push(read_adv!(u32, slice, pointer));
    }

    game_info
}