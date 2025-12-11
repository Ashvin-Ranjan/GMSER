use log::{info, warn};
use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
    sequence::{PlaybackSpeedType, Sequence},
    texture::TextureItem,
};

#[derive(Debug)]
pub enum TileMode {
    Stretch,
    Repeat,
    Mirror,
    BlankRepeat,
    Hide,
}

#[derive(Debug)]
pub struct NineSlice {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub enabled: bool,
    pub tile_modes: [TileMode; 5],
}

#[derive(Debug)]
pub enum SepMasksType {
    AxisAlrignedRect,
    Precise,
    RotatedRect,
}

#[derive(Debug)]
pub struct SpriteSequence {
    pub sequence: Sequence,
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub margin_left: u32,
    pub margin_right: u32,
    pub margin_bottom: u32,
    pub margin_top: u32,
    pub transparent: bool,
    pub smooth: bool,
    pub preload: bool,
    pub bbox_mode: u32,
    pub sep_masks: SepMasksType,
    pub origin_x: u32,
    pub origin_y: u32,
    pub playback_speed: f32,
    pub playback_speed_type: PlaybackSpeedType,
    pub sequence: Option<SpriteSequence>,
    pub nine_slice: Option<NineSlice>,
    pub textures: Vec<TextureItem>,
}

#[derive(Debug)]
pub struct SprtChunk {
    pub size: u32,
    pub sprites: Vec<Sprite>,
}

impl SprtChunk {
    const IDENT: [u8; 4] = [0x53, 0x50, 0x52, 0x54]; // "SPRT"
}

pub fn deserialize_sprt(cursor: &mut Cursor<&[u8]>) -> Result<SprtChunk, DataLoadError> {
    info!("Deserializing SPRT");

    let ident = cursor.read_ident()?;

    if ident != SprtChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: SprtChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let sprites = cursor.read_pointer_list::<Sprite>(0)?;

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(SprtChunk { size, sprites })
}

impl Deserializable for Sprite {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let width = cursor.read_u32()?;
        let height = cursor.read_u32()?;
        let margin_left = cursor.read_u32()?;
        let margin_right = cursor.read_u32()?;
        let margin_bottom = cursor.read_u32()?;
        let margin_top = cursor.read_u32()?;
        let transparent = cursor.read_u32()? != 0;
        let smooth = cursor.read_u32()? != 0;
        let preload = cursor.read_u32()? != 0;
        let bbox_mode = cursor.read_u32()?;

        let sep_masks_num = cursor.read_u32()?;
        let sep_masks = match sep_masks_num {
            0 => Ok(SepMasksType::AxisAlrignedRect),
            1 => Ok(SepMasksType::Precise),
            2 => Ok(SepMasksType::RotatedRect),
            _ => Err(DataLoadError::InvalidSepMasksType {
                mask_type: sep_masks_num,
                pos: cursor.position() - 4,
            }),
        }?;

        let origin_x = cursor.read_u32()?;
        let origin_y = cursor.read_u32()?;

        let gm2_sprite_type = cursor.read_u32()?;
        if gm2_sprite_type != 0xFFFFFFFF {
            return Err(DataLoadError::OutdatedSpriteType {
                sprite_type: gm2_sprite_type,
                pos: cursor.position() - 4,
            });
        }

        let version = cursor.read_u32()?;
        if version < 3 {
            return Err(DataLoadError::OutdatedSpriteVersion {
                sprite_version: version,
                pos: cursor.position() - 4,
            });
        }

        let sprite_type = cursor.read_u32()?;

        let playback_speed = cursor.read_f32()?;

        let playback_speed_type = PlaybackSpeedType::deserialize(cursor)?;

        let sequence = cursor.read_opt_pointer::<SpriteSequence>(0)?;

        let nine_slice = cursor.read_opt_pointer::<NineSlice>(0)?;

        if sprite_type != 0 {
            return Err(DataLoadError::DebugError {
                message: format!("Unimplemented sprite type: `{}`", sprite_type),
                pos: cursor.position(),
            });
        }

        let textures = cursor.read_pointer_list::<TextureItem>(0)?;

        Ok(Sprite {
            name,
            width,
            height,
            margin_left,
            margin_right,
            margin_bottom,
            margin_top,
            transparent,
            smooth,
            preload,
            bbox_mode,
            sep_masks,
            origin_x,
            origin_y,
            playback_speed,
            playback_speed_type,
            sequence,
            nine_slice,
            textures,
        })
    }
}

impl Deserializable for SpriteSequence {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let format_id = cursor.read_u32()?;
        if format_id != 1 {
            warn!(
                "Encountered sprite sequence format id {}, treating as 1",
                format_id
            );
        }

        Ok(SpriteSequence {
            sequence: Sequence::deserialize(cursor)?,
        })
    }
}

impl Deserializable for NineSlice {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let left = cursor.read_u32()?;
        let top = cursor.read_u32()?;
        let right = cursor.read_u32()?;
        let bottom = cursor.read_u32()?;
        let enabled = cursor.read_u32()? != 0;
        let mut tile_modes = [const { TileMode::Hide }; 5];
        for i in 0..5 {
            let mode = cursor.read_u32()?;
            tile_modes[i] = match mode {
                0 => Ok(TileMode::Stretch),
                1 => Ok(TileMode::Repeat),
                2 => Ok(TileMode::Mirror),
                3 => Ok(TileMode::BlankRepeat),
                4 => Ok(TileMode::Hide),
                _ => Err(DataLoadError::InvalidTileMode {
                    mode,
                    pos: cursor.position() - 4,
                }),
            }?;
        }

        Ok(NineSlice {
            left,
            top,
            right,
            bottom,
            enabled,
            tile_modes,
        })
    }
}
