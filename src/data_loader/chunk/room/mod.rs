mod layer;
mod utils;

use bitflags::bitflags;
use log::{info, warn};
use snafu::OptionExt;
use std::io::Cursor;

use crate::data_loader::{
    chunk::room::{
        layer::Layer,
        utils::{read_pointer_list_ref, read_u32_list_ref, Background, GameObject, Tile, View},
    },
    utils::{
        cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
        error::{DataLoadError, InvalidRoomFlagsSnafu},
    },
};

bitflags! {
    #[derive(Debug)]
    pub struct RoomFlags: u32 {
        const ENABLE_VIEWS         = 0x1;
        const SHOW_COLOR           = 0x2;
        const CLEAR_DISPLAY_BUFFER = 0x64;
    }
}

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub caption: Option<String>,
    pub width: u32,
    pub height: u32,
    pub speed: u32,
    pub persistent: bool,
    pub background_color: u32,
    pub draw_background_color: bool,
    pub creation_code_id: u32,
    pub room_flags: RoomFlags,
    pub backgrounds: Vec<Background>,
    pub views: Vec<View>,
    pub game_objects: Vec<GameObject>,
    pub tiles: Vec<Tile>,
    pub physics: bool,
    pub top: u32,
    pub left: u32,
    pub right: u32,
    pub bottom: u32,
    pub gravity_x: f32,
    pub gravity_y: f32,
    pub pixels_to_meters: f32,
    pub layers: Vec<Layer>,
    pub sequence_ids: Vec<u32>,
}

#[derive(Debug)]
pub struct RoomChunk {
    pub size: u32,
    pub rooms: Vec<Room>,
}

impl RoomChunk {
    const IDENT: [u8; 4] = [0x52, 0x4F, 0x4F, 0x4D]; // "ROOM"
}

impl Deserializable for RoomChunk {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        info!("Deserializing ROOM");

        let ident = cursor.read_ident()?;

        if ident != RoomChunk::IDENT {
            return Err(DataLoadError::UnexpectedIdent {
                pos: cursor.position() - 4,
                expected: RoomChunk::IDENT,
                actual: ident,
            });
        }

        let size = cursor.read_u32()?;

        let start_pos = cursor.position();

        let rooms = cursor.read_pointer_list::<Room>(0)?;

        handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

        warn!("We currently do not handle deserialization for various layer types!");

        Ok(RoomChunk { size, rooms })
    }
}

impl Deserializable for Room {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let caption = cursor.read_opt_pointer::<String>(0)?;
        let width = cursor.read_u32()?;
        let height = cursor.read_u32()?;
        let speed = cursor.read_u32()?;
        let persistent = cursor.read_wide_boolean()?;
        let background_color = cursor.read_u32()?;
        let draw_background_color = cursor.read_wide_boolean()?;
        let creation_code_id = cursor.read_u32()?;
        let room_flags_number = cursor.read_u32()? & !0x30000u32;
        let room_flags =
            RoomFlags::from_bits(room_flags_number).context(InvalidRoomFlagsSnafu {
                pos: cursor.position() - 4,
                flag: room_flags_number,
            })?;

        // This chunk has pointers to lists of other values, store those pointers and handle them later
        let backgrounds = read_pointer_list_ref::<Background>(cursor)?;
        let views = read_pointer_list_ref::<View>(cursor)?;
        let game_objects = read_pointer_list_ref::<GameObject>(cursor)?;
        let tiles = read_pointer_list_ref::<Tile>(cursor)?;

        let physics = cursor.read_wide_boolean()?;
        let top = cursor.read_u32()?;
        let left = cursor.read_u32()?;
        let right = cursor.read_u32()?;
        let bottom = cursor.read_u32()?;
        let gravity_x = cursor.read_f32()?;
        let gravity_y = cursor.read_f32()?;
        let pixels_to_meters = cursor.read_f32()?;

        let layers = read_pointer_list_ref::<Layer>(cursor)?;
        let sequence_ids = read_u32_list_ref(cursor)?;

        Ok(Room {
            name,
            caption,
            width,
            height,
            speed,
            persistent,
            background_color,
            draw_background_color,
            creation_code_id,
            room_flags,
            backgrounds,
            views,
            game_objects,
            tiles,
            physics,
            top,
            left,
            right,
            bottom,
            gravity_x,
            gravity_y,
            pixels_to_meters,
            layers,
            sequence_ids,
        })
    }
}
