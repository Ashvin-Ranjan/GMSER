use log::{info, warn};
use snafu::ResultExt;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use crate::data_loader::utils::{
    cursor::{handle_cursor_alignment, CustomCursor},
    error::{DataLoadError, IOSnafu},
};

#[derive(Debug)]
pub struct AudoChunk {
    pub size: u32,
    pub audio_map: HashMap<u32, Vec<u8>>,
}

impl AudoChunk {
    const IDENT: [u8; 4] = [0x41, 0x55, 0x44, 0x4F]; // "AUDO"
}

pub fn deserialize_audo(cursor: &mut Cursor<&[u8]>) -> Result<AudoChunk, DataLoadError> {
    info!("Deserializing AUDO");

    let ident = cursor.read_ident()?;

    if ident != AudoChunk::IDENT {
        return Err(DataLoadError::UnexpectedIdent {
            pos: cursor.position() - 4,
            expected: AudoChunk::IDENT,
            actual: ident,
        });
    }

    let size = cursor.read_u32()?;

    let start_pos = cursor.position();

    let audio_map = cursor.read_pointer_map(deserialize_audio, 0)?;

    warn!("We currently do not load the audio group files!");

    handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

    Ok(AudoChunk { size, audio_map })
}

fn deserialize_audio(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, DataLoadError> {
    let size = cursor.read_u32()?;
    let start_pos = cursor.position();
    let mut audio_buffer = vec![0u8; size as usize];
    cursor
        .read_exact(&mut audio_buffer)
        .context(IOSnafu { pos: start_pos })?;
    Ok(audio_buffer)
}
