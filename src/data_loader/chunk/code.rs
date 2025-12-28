use log::info;
use snafu::ResultExt;
use std::io::{Cursor, Read};

use crate::data_loader::utils::{
    chunk::Chunk,
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::{DataLoadError, IOSnafu},
};

#[derive(Debug)]
pub struct CodeEntry {
    pub name: String,
    pub length: u32,
    pub locals_count: u16,
    pub arguments_count: u16,
    pub unk_flags: u8,
    pub bytecode: Vec<u8>,
}

#[derive(Debug)]
pub struct CodeChunk {
    pub size: u32,
    pub code_entries: Vec<CodeEntry>,
}

impl Chunk for CodeChunk {
    const IDENT: [u8; 4] = [0x43, 0x4F, 0x44, 0x45]; // "CODE"
}

impl Deserializable for CodeChunk {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        info!("Deserializing CODE");

        let ident = cursor.read_ident()?;

        if ident != CodeChunk::IDENT {
            return Err(DataLoadError::UnexpectedIdent {
                pos: cursor.position() - 4,
                expected: CodeChunk::IDENT,
                actual: ident,
            });
        }

        let size = cursor.read_u32()?;

        let start_pos = cursor.position();

        let code_entries = cursor.read_pointer_list::<CodeEntry>(0)?;

        handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

        Ok(CodeChunk { size, code_entries })
    }
}

impl Deserializable for CodeEntry {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let length = cursor.read_u32()?;
        let locals_count = cursor.read_u16()?;
        let args_flags = cursor.read_u16()?;
        let arguments_count = args_flags & 0b1111111111111u16;
        let unk_flags = (args_flags >> 13) as u8;

        // Probably should make this better since this is not commutative
        // Weird things going on with type conversion because the relative address can be negative
        let bytecode_addr = (cursor.position() as i32 + cursor.read_i32()?) as u64;
        let mut bytecode = vec![0u8; length as usize];
        cursor.set_position(bytecode_addr);
        cursor
            .read_exact(&mut bytecode)
            .context(IOSnafu { pos: bytecode_addr })?;

        Ok(CodeEntry {
            name,
            length,
            locals_count,
            arguments_count,
            unk_flags,
            bytecode,
        })
    }
}
