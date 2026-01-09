//! Contains structs for sequences used in [`crate::data_loader::chunk::room`] and [`crate::data_loader::chunk::sprt`]
//!
//! # Notes
//! - This file is currently unfinished as sequence deserialization is not needed for now.

use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{
    cursor::{CustomCursor, Deserializable},
    error::DataLoadError,
};

/// Indicates a type of playback speed used for animations
#[derive(Debug)]
pub enum PlaybackSpeedType {
    /// Indicates that the playback speed for an animation is in frames-per-second.
    ///
    /// # Notes
    /// - Represented by numerical value `0`
    FramesPerSecond,
    /// Indicates that the playback speed for an animation is in frames-per-game frame.
    ///
    /// # Notes
    /// - Represented by numerical value `1`
    FramesPerGameFrame,
}

/// Indicates a playback type for a [`Sequence`].
///
/// # Notes
/// - Currently unused.
#[derive(Debug)]
pub enum PlaybackType {
    Oneshot,
    Loop,
    PingPong,
}

/// Indicates a keyframe in a [`Sequence`].
///
/// # Notes
/// - Currently unused.
#[derive(Debug)]
pub struct Keyframe<T>
where
    T: Deserializable,
{
    pub key: f32,
    pub length: f32,
    pub stretched: bool,
    pub disabled: bool,
    pub channels: HashMap<u32, T>,
}

/// Indicates a moment in a [`Sequence`].
///
/// # Notes
/// - Currently unused.
/// - Used as a generic type for [`Keyframe`].
#[derive(Debug)]
pub struct Moment {
    pub internal_count: u32,
    pub event: Option<String>,
}

/// Indicates a sequence of events.
///
/// # Notes
/// - Currently unused.
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

impl Deserializable for Moment {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        Err(DataLoadError::DebugError {
            message: "Moment deserialization is not implemented!".to_owned(),
            pos: cursor.position(),
        })
    }
}

impl<T> Deserializable for Keyframe<T>
where
    T: Deserializable,
{
    /// Deserialization for [`Keyframe`].
    ///
    /// # Format Specification
    /// Deserialization of [`Keyframe`] is done by reading in all of the
    /// fields in order. Note that [`Keyframe::stretched`] and [`Keyframe::disabled`] are
    /// wide booleans, meaning they are 4 bytes long. Furthermore, [`Keyframe::channels`]
    /// is not serialized as a pointer list but as a sequential list. As such, first the count
    /// (an unsigned 4-byte integer) must be read and then values of type `T` deserialized
    /// sequentially.
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let key = cursor.read_f32()?;
        let length = cursor.read_f32()?;
        let stretched = cursor.read_wide_boolean()?;
        let disabled = cursor.read_wide_boolean()?;

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
    /// Deserialization for [`PlaybackSpeedType`]
    ///
    /// # Format Specification
    /// The speed type is serialized as a [`u32`] with numerical values documented
    /// specifically in [`PlaybackSpeedType`]'s fields.
    ///
    /// # Output
    /// If an invalid speed type is read, returns a [`DataLoadError::InvalidPlaybackSpeedType`].
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
