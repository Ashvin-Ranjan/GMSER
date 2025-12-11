use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{
    cursor::{CustomCursor, Deserializable},
    error::DataLoadError,
};

#[derive(Debug)]
pub enum PlaybackSpeedType {
    FramesPerSecond,
    FramesPerGameFrame,
}

#[derive(Debug)]
pub enum PlaybackType {
    Oneshot,
    Loop,
    PingPong,
}

#[derive(Debug)]
pub struct Keyframe<T> {
    pub key: f32,
    pub length: f32,
    pub stretched: bool,
    pub disabled: bool,
    pub channels: HashMap<u32, T>,
}

#[derive(Debug)]
pub struct Moment {
    pub internal_count: u32,
    pub event: Option<String>,
}

#[derive(Debug)]
pub struct Sequence {
    pub name: String,
    pub playback_type: PlaybackType,
    pub playback_speed: f32,
    pub playback_speed_type: PlaybackSpeedType,
    pub length: f32,
    pub origin_x: u32,
    pub origin_y: u32,
    pub volume: f32,
    pub broadcast_messages: Vec<Keyframe<String>>,
    // pub tracks: Vec<Track>, TODO: Later
    pub function_ids: HashMap<u32, String>,
    pub moments: Vec<Keyframe<Moment>>,
}

impl Deserializable for Sequence {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        Err(DataLoadError::DebugError {
            message: "Sequence deserialization is not implemented!".to_owned(),
            pos: cursor.position(),
        })
    }
}

impl<T> Deserializable for Keyframe<T>
where
    T: Deserializable,
{
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let key = cursor.read_f32()?;
        let length = cursor.read_f32()?;
        let stretched = cursor.read_u32()? != 0;
        let disabled = cursor.read_u32()? != 0;

        let count = cursor.read_u32()?;
        let mut channels = HashMap::new();
        for _ in 0..count {
            let channel = cursor.read_u32()?;
            let data = T::deserialize(cursor)?;
            channels.insert(channel, data);
        }

        Ok(Keyframe {
            key,
            length,
            stretched,
            disabled,
            channels,
        })
    }
}

impl Deserializable for PlaybackSpeedType {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let speed_type = cursor.read_u32()?;
        match speed_type {
            0 => Ok(PlaybackSpeedType::FramesPerSecond),
            1 => Ok(PlaybackSpeedType::FramesPerGameFrame),
            _ => Err(DataLoadError::InvalidPlaybackSpeedType {
                speed_type: speed_type,
                pos: cursor.position() - 4,
            }),
        }
    }
}
