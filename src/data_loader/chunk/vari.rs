use log::{info, warn};
use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{
    chunk::Chunk,
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
    reference_chains::{resolve_reference_chain, Reference},
};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub variable_type: u32,
    pub id: u32,
    pub first_occurrence: u32,
    pub occurrences: u32,
}

#[derive(Debug)]
pub struct VariChunk {
    pub size: u32,
    pub var_count: u32,
    pub max_local_var_count: u32,
    pub variables: Vec<Variable>,
    pub reference_map: HashMap<u64, Reference>,
}

impl Chunk for VariChunk {
    const IDENT: [u8; 4] = [0x56, 0x41, 0x52, 0x49]; // "VARI"
}

impl Deserializable for VariChunk {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        info!("Deserializing VARI");

        let ident = cursor.read_ident()?;

        if ident != VariChunk::IDENT {
            return Err(DataLoadError::UnexpectedIdent {
                pos: cursor.position() - 4,
                expected: VariChunk::IDENT,
                actual: ident,
            });
        }

        let size = cursor.read_u32()?;

        let start_pos = cursor.position();

        let var_count_1 = cursor.read_u32()?;
        let var_count_2 = cursor.read_u32()?;
        if var_count_1 != var_count_2 {
            warn!(
                "var_count_1 ({}) is not equal to var_count_2 ({})",
                var_count_1, var_count_2
            );
        }
        let var_count = if var_count_1 > var_count_2 {
            var_count_1
        } else {
            var_count_2
        };

        let max_local_var_count = cursor.read_u32()?;

        let mut variables = Vec::new();
        let mut index = 0;
        let mut reference_map = HashMap::new();
        while start_pos + size as u64 - cursor.position() >= 20 {
            let newest_var = Variable::deserialize(cursor)?;
            if newest_var.occurrences != 0 {
                resolve_reference_chain(
                    cursor,
                    &newest_var.occurrences,
                    index,
                    &newest_var.first_occurrence,
                    &mut reference_map,
                )?;
            }
            variables.push(newest_var);
            index += 1;
        }

        if variables.len() as u32 != var_count {
            warn!(
                "var_count is {} but only {} variables were loaded",
                var_count,
                variables.len()
            )
        }

        handle_cursor_alignment(cursor, start_pos, size as u64, false)?;

        Ok(VariChunk {
            size,
            var_count,
            max_local_var_count,
            variables,
            reference_map,
        })
    }
}

impl Deserializable for Variable {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let variable_type = cursor.read_u32()?;
        let id = cursor.read_u32()?;
        let occurrences = cursor.read_u32()?;
        let first_occurrence = cursor.read_u32()?;
        if occurrences == 0 {
            if first_occurrence != 0xFFFFFFFFu32 {
                warn!(
                    "Variable {} has no occurrences but lists first occurrence at {}.",
                    name, first_occurrence
                );
            }
        }

        Ok(Variable {
            name,
            variable_type,
            id,
            first_occurrence,
            occurrences,
        })
    }
}
