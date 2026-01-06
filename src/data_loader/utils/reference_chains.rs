use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{cursor::CustomCursor, error::DataLoadError};

#[derive(Debug)]
pub struct Reference {
    pub id: u32,
    pub ref_type: u32,
}

const REFERENCE_LOCATION_MASK: u32 = 0x07FFFFFF;

pub fn resolve_reference_chain(
    cursor: &mut Cursor<&[u8]>,
    occurrences: &u32,
    id: u32,
    first_occurrence: &u32,
    reference_map: &mut HashMap<u64, Reference>,
) -> Result<(), DataLoadError> {
    let start_position = cursor.position();
    cursor.set_position(*first_occurrence as u64 + 4);
    for _ in 0..*occurrences {
        let reference_position = cursor.position();
        let reference_information = cursor.read_u32()?;
        reference_map.insert(
            reference_position,
            Reference {
                id,
                ref_type: (reference_information & !REFERENCE_LOCATION_MASK) >> 24,
            },
        );
        cursor.set_position(
            reference_position + (reference_information & REFERENCE_LOCATION_MASK) as u64,
        );
    }
    cursor.set_position(start_position);
    Ok(())
}
