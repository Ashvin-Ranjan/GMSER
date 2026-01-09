//! This module contains a broad trait used to help create flexible chunk deserialization.

use crate::data_loader::utils::cursor::Deserializable;

/// Trait for a chunk which has an identifier.
/// # Notes
/// - This is only here for better extensibility with loading in the FORM chunk.
/// - IDENT is a `[u8; 4]` because it is the simplest way in Rust to handle 4 bytes.
pub trait Chunk
where
    Self: Deserializable,
{
    const IDENT: [u8; 4];
}
