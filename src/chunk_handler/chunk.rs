use crate::{FILE_DATA, read_num, utils::error};

use std::str;

pub struct Chunk {
    pub ident: String,
    pub size: u32,
    pub start: u32,
}

fn read_chunk_data(start: usize) -> Chunk {
    Chunk {
        ident: str::from_utf8(&FILE_DATA.as_slice()[start..(start+4)]).expect("Unable to parse first bytes").to_owned(),
        size: read_num!(u32, FILE_DATA.as_slice(), start+4) + 8, // The ident and size are not included in the size
        start: start as u32
    }
}

pub fn locate_chunks() -> Result<Vec<Chunk>, error::ChunkParseError> {
    let mut out = Vec::new();

    let form = read_chunk_data(0);

    if form.ident != "FORM" || form.size != FILE_DATA.len() as u32 {
        return Err(error::ChunkParseError::InvalidFormChunk);
    }

    out.push(form);

    let mut pointer = 8;

    while pointer < FILE_DATA.len() {
        let chunk = read_chunk_data(pointer);
        pointer += chunk.size as usize;
        out.push(chunk);
    }

    Ok(out)
}