use log::info;
use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

#[derive(Debug)]
pub struct Path {
    pub name: String,
    pub smooth: bool,
    pub closed: bool,
    pub precision: u32,
    pub points: Vec<Point>,
}

#[derive(Debug)]
pub struct PathChunk {
    pub size: u32,
    pub path_map: HashMap<u32, Path>,
}

impl PathChunk {
    const IDENT: [u8; 4] = [0x50, 0x41, 0x54, 0x48]; // "PATH"
}

pub fn deserialize_path(cursor: &mut Cursor<&[u8]>) -> Result<PathChunk, DataLoadError> {
    info!("Deserializing PATH");

    let ident = cursor.read_ident()?;

    if ident != PathChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: PathChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let path_map = cursor.read_pointer_map::<Path>(0)?;

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(PathChunk { size, path_map })
}

impl Deserializable for Path {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let smooth = cursor.read_u32()? != 0;
        let closed = cursor.read_u32()? != 0;
        let precision = cursor.read_u32()?;

        let num_points = cursor.read_u32()?;
        let mut points = Vec::new();
        for _ in 0..num_points {
            let x = cursor.read_f32()?;
            let y = cursor.read_f32()?;
            let speed = cursor.read_f32()?;
            points.push(Point { x, y, speed })
        }

        Ok(Path {
            name,
            smooth,
            closed,
            precision,
            points,
        })
    }
}
