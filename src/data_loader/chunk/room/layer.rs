use std::io::Cursor;

use log::info;

use crate::data_loader::{
    chunk::room::utils::Tile,
    utils::{
        cursor::{CustomCursor, Deserializable},
        error::DataLoadError,
        sequence::PlaybackSpeedType,
    },
};

#[derive(Debug)]
pub struct Layer {
    pub name: String,
    pub id: u32,
    pub depth: i32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub h_speed: f32,
    pub v_speed: f32,
    pub visible: bool,
    pub effect_enabled: bool,
    pub effect_type: Option<String>,
    pub effect_properties: Vec<EffectProperty>,
    pub layer_kind: LayerKind,
}

#[derive(Debug)]
pub struct EffectProperty {
    pub property_type: PropertyType,
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub enum PropertyType {
    REAL,
    COLOR,
    SAMPLER,
}

#[derive(Debug)]
pub enum LayerKind {
    Background(LayerBackground),
    Instances(LayerInstances),
    Assets(LayerAssets),
    Tiles(LayerTiles),
    Effect(LayerEffect),
}

#[derive(Debug)]
pub struct LayerBackground {
    pub visible: bool,
    pub foreground: bool,
    pub sprite_id: u32,
    pub tile_h: bool,
    pub tile_v: bool,
    pub stretch: bool,
    pub color: u32,
    pub first_frame: f32,
    pub animation_speed: f32,
    pub animation_speed_type: PlaybackSpeedType,
}

#[derive(Debug)]
pub struct LayerInstances {
    pub instances: Vec<u32>,
}

#[derive(Debug)]
pub struct LayerAssets {
    pub legacy_tiles: Vec<Tile>,
    pub sprites: Vec<AssetInstance>,
    pub sequences: Vec<AssetInstance>,
}

#[derive(Debug)]
pub struct AssetInstance {
    pub name: String,
    pub asset_id: u32,
    pub x: i32,
    pub y: i32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub color: u32,
    pub animation_speed: f32,
    pub animation_speed_type: PlaybackSpeedType,
    pub frame_index: f32,
    pub rotation: f32,
}

#[derive(Debug)]
pub struct LayerTiles {
    pub background_id: u32,
    pub tiles_x: u32,
    pub tiles_y: u32,
    pub tile_data: Vec<Vec<u32>>,
}

#[derive(Debug)]
pub struct LayerEffect {
    pub effect_type: String,
    pub effect_properties: Vec<EffectProperty>,
}

impl Deserializable for Layer {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let id = cursor.read_u32()?;
        let layer_kind_pos = cursor.position();
        let kind = cursor.read_u32()?;
        let depth = cursor.read_i32()?;
        let offset_x = cursor.read_f32()?;
        let offset_y = cursor.read_f32()?;
        let h_speed = cursor.read_f32()?;
        let v_speed = cursor.read_f32()?;
        let visible = cursor.read_u32()? != 0;
        let effect_enabled = cursor.read_u32()? != 0;
        let effect_type = cursor.read_opt_pointer::<String>(0)?;
        let effect_properties = cursor.read_pointer_list::<EffectProperty>(0)?;
        let layer_kind = match kind {
            1 => Ok(LayerKind::Background(LayerBackground::deserialize(cursor)?)),
            2 => Ok(LayerKind::Instances(LayerInstances::deserialize(cursor)?)),
            3 => Ok(LayerKind::Assets(LayerAssets::deserialize(cursor)?)),
            4 => Ok(LayerKind::Tiles(LayerTiles::deserialize(cursor)?)),
            6 => Ok(LayerKind::Effect(LayerEffect::deserialize(cursor)?)),
            _ => Err(DataLoadError::InvalidLayerKind {
                kind,
                pos: layer_kind_pos,
            }),
        }?;

        Ok(Layer {
            name,
            id,
            depth,
            offset_x,
            offset_y,
            h_speed,
            v_speed,
            visible,
            effect_enabled,
            effect_type,
            effect_properties,
            layer_kind,
        })
    }
}

impl Deserializable for EffectProperty {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let kind_index = cursor.read_u32()?;
        let property_type = match kind_index {
            0 => Ok(PropertyType::REAL),
            1 => Ok(PropertyType::COLOR),
            2 => Ok(PropertyType::SAMPLER),
            _ => Err(DataLoadError::InvalidEffectPropertyKind {
                pos: cursor.position() - 4,
                kind: kind_index,
            }),
        }?;
        let name = cursor.read_string()?;
        let value = cursor.read_string()?;

        Ok(EffectProperty {
            property_type,
            name,
            value,
        })
    }
}

impl Deserializable for LayerBackground {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let visible = cursor.read_u32()? != 0;
        let foreground = cursor.read_u32()? != 0;
        let sprite_id = cursor.read_u32()?;
        let tile_h = cursor.read_u32()? != 0;
        let tile_v = cursor.read_u32()? != 0;
        let stretch = cursor.read_u32()? != 0;
        let color = cursor.read_u32()?;
        let first_frame = cursor.read_f32()?;
        let animation_speed = cursor.read_f32()?;
        let animation_speed_type = PlaybackSpeedType::deserialize(cursor)?;

        Ok(LayerBackground {
            visible,
            foreground,
            sprite_id,
            tile_h,
            tile_v,
            stretch,
            color,
            first_frame,
            animation_speed,
            animation_speed_type,
        })
    }
}

impl Deserializable for LayerInstances {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let instances_count = cursor.read_u32()?;
        let mut instances = Vec::new();
        for _ in 0..instances_count {
            instances.push(cursor.read_u32()?);
        }

        Ok(LayerInstances { instances })
    }
}

impl Deserializable for LayerAssets {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        todo!("Implement Deserializable for LayerAssets")
    }
}

impl Deserializable for LayerTiles {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        todo!("Implement Deserializable for LayerTiles")
    }
}

impl Deserializable for LayerEffect {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        todo!("Implement Deserializable for LayerEffect")
    }
}
