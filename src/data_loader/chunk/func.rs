use log::{info, warn};
use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{
    chunk::Chunk,
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
    reference_chains::{resolve_reference_chain, Reference},
};

#[derive(Debug)]
pub struct FunctionEntry {
    pub name: String,
    pub first_occurrence: u32,
    pub occurrences: u32,
}

#[derive(Debug)]
pub struct LocalVariable {
    pub name: String,
    pub index: u32,
}

#[derive(Debug)]
pub struct CodeLocal {
    pub name: String,
    pub locals: Vec<LocalVariable>,
}

#[derive(Debug)]
pub struct FuncChunk {
    pub size: u32,
    pub function_entries: Vec<FunctionEntry>,
    pub reference_map: HashMap<u64, Reference>,
    pub code_locals: Vec<CodeLocal>,
}

impl Chunk for FuncChunk {
    const IDENT: [u8; 4] = [0x46, 0x55, 0x4E, 0x43]; // "FUNC"
}

impl Deserializable for FuncChunk {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        info!("Deserializing FUNC");

        let ident = cursor.read_ident()?;

        if ident != FuncChunk::IDENT {
            return Err(DataLoadError::UnexpectedIdent {
                pos: cursor.position() - 4,
                expected: FuncChunk::IDENT,
                actual: ident,
            });
        }

        let size = cursor.read_u32()?;

        let start_pos = cursor.position();

        let function_entry_count = cursor.read_u32()?;
        let mut function_entries = Vec::new();
        let mut reference_map = HashMap::new();
        for i in 0..function_entry_count {
            let newest_var = FunctionEntry::deserialize(cursor)?;
            if newest_var.occurrences != 0 {
                resolve_reference_chain(
                    cursor,
                    &newest_var.occurrences,
                    i,
                    &(&newest_var.first_occurrence - 4),
                    &mut reference_map,
                )?;
            }
            function_entries.push(newest_var);
        }

        let code_locals_count = cursor.read_u32()?;
        let mut code_locals = Vec::new();
        for _ in 0..code_locals_count {
            code_locals.push(CodeLocal::deserialize(cursor)?);
        }

        handle_cursor_alignment(cursor, start_pos, size as u64, false)?;

        Ok(FuncChunk {
            size,
            function_entries,
            reference_map,
            code_locals,
        })
    }
}

impl Deserializable for FunctionEntry {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let occurrences = cursor.read_u32()?;
        let first_occurrence = cursor.read_u32()?;

        Ok(FunctionEntry {
            name,
            first_occurrence,
            occurrences,
        })
    }
}

impl Deserializable for CodeLocal {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let locals_count = cursor.read_u32()?;
        let name = cursor.read_obj_pointer::<String>(0)?;
        let mut locals = Vec::new();
        for _ in 0..locals_count {
            locals.push(LocalVariable::deserialize(cursor)?);
        }

        Ok(CodeLocal { name, locals })
    }
}

impl Deserializable for LocalVariable {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let index = cursor.read_u32()?;
        let name = cursor.read_obj_pointer::<String>(0)?;

        Ok(LocalVariable { name, index })
    }
}
