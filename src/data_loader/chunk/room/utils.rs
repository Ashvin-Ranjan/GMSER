use std::io::Cursor;

use crate::data_loader::utils::{
    cursor::{CustomCursor, Deserializable},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct Background {
    pub enabled: bool,
    pub foreground: bool,
    pub background_id: u32,
    pub x: i32,
    pub y: i32,
    pub tile_x: i32,
    pub tile_y: i32,
    pub speed_x: i32,
    pub speed_y: i32,
    pub stretch: bool,
}

#[derive(Debug)]
pub struct View {
    pub enabled: bool,
    pub view_x: i32,
    pub view_y: i32,
    pub view_width: u32,
    pub view_height: u32,
    pub port_x: i32,
    pub port_y: i32,
    pub port_width: u32,
    pub port_height: u32,
    pub border_x: i32,
    pub border_y: i32,
    pub speed_x: i32,
    pub speed_y: i32,
    pub follow_object_id: u32,
}

#[derive(Debug)]
pub struct GameObject {
    pub x: i32,
    pub y: i32,
    pub object_id: u32,
    pub instance_id: u32,
    pub creation_code_id: u32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub color: u32,
    pub angle: f32,
    pub precreate_code_id: u32,
    pub image_speed: f32,
    pub image_index: u32,
}

#[derive(Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub asset_id: u32,
    pub source_x: i32,
    pub source_y: i32,
    pub width: u32,
    pub height: u32,
    pub depth: i32,
    pub id: u32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub color: u32,
}

// This chunk references pointer lists, which we need to handle
pub fn read_pointer_list_ref<T>(cursor: &mut Cursor<&[u8]>) -> Result<Vec<T>, DataLoadError>
where
    T: Deserializable,
{
    let original_pos = cursor.position();
    let move_pos = cursor.read_u32()? as u64;
    cursor.set_position(move_pos);
    let output_list = cursor.read_pointer_list(0);
    cursor.set_position(original_pos + 4);
    output_list
}

pub fn read_u32_list_ref(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u32>, DataLoadError> {
    let original_pos = cursor.position();
    let move_pos = cursor.read_u32()? as u64;
    cursor.set_position(move_pos);
    let length = cursor.read_u32()?;
    let mut output = Vec::new();
    for _ in 0..length {
        output.push(cursor.read_u32()?)
    }
    cursor.set_position(original_pos + 4);
    Ok(output)
}

impl Deserializable for Background {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let enabled = cursor.read_u32()? != 0;
        let foreground = cursor.read_u32()? != 0;
        let background_id = cursor.read_u32()?;
        let x = cursor.read_i32()?;
        let y = cursor.read_i32()?;
        let tile_x = cursor.read_i32()?;
        let tile_y = cursor.read_i32()?;
        let speed_x = cursor.read_i32()?;
        let speed_y = cursor.read_i32()?;
        let stretch = cursor.read_u32()? != 0;

        Ok(Background {
            enabled,
            foreground,
            background_id,
            x,
            y,
            tile_x,
            tile_y,
            speed_x,
            speed_y,
            stretch,
        })
    }
}

impl Deserializable for View {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let enabled = cursor.read_u32()? != 0;
        let view_x = cursor.read_i32()?;
        let view_y = cursor.read_i32()?;
        let view_width = cursor.read_u32()?;
        let view_height = cursor.read_u32()?;
        let port_x = cursor.read_i32()?;
        let port_y = cursor.read_i32()?;
        let port_width = cursor.read_u32()?;
        let port_height = cursor.read_u32()?;
        let border_x = cursor.read_i32()?;
        let border_y = cursor.read_i32()?;
        let speed_x = cursor.read_i32()?;
        let speed_y = cursor.read_i32()?;
        let follow_object_id = cursor.read_u32()?;

        Ok(View {
            enabled,
            view_x,
            view_y,
            view_width,
            view_height,
            port_x,
            port_y,
            port_width,
            port_height,
            border_x,
            border_y,
            speed_x,
            speed_y,
            follow_object_id,
        })
    }
}

impl Deserializable for GameObject {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let x = cursor.read_i32()?;
        let y = cursor.read_i32()?;
        let object_id = cursor.read_u32()?;
        let instance_id = cursor.read_u32()?;
        let creation_code_id = cursor.read_u32()?;
        let scale_x = cursor.read_f32()?;
        let scale_y = cursor.read_f32()?;
        let image_speed = cursor.read_f32()?;
        let image_index = cursor.read_u32()?;
        let color = cursor.read_u32()?;
        let angle = cursor.read_f32()?;
        let precreate_code_id = cursor.read_u32()?;

        Ok(GameObject {
            x,
            y,
            object_id,
            instance_id,
            creation_code_id,
            scale_x,
            scale_y,
            color,
            angle,
            precreate_code_id,
            image_speed,
            image_index,
        })
    }
}

impl Deserializable for Tile {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let x = cursor.read_i32()?;
        let y = cursor.read_i32()?;
        let asset_id = cursor.read_u32()?;
        let source_x = cursor.read_i32()?;
        let source_y = cursor.read_i32()?;
        let width = cursor.read_u32()?;
        let height = cursor.read_u32()?;
        let depth = cursor.read_i32()?;
        let id = cursor.read_u32()?;
        let scale_x = cursor.read_f32()?;
        let scale_y = cursor.read_f32()?;
        let color = cursor.read_u32()?;

        Ok(Tile {
            x,
            y,
            asset_id,
            source_x,
            source_y,
            width,
            height,
            depth,
            id,
            scale_x,
            scale_y,
            color,
        })
    }
}
