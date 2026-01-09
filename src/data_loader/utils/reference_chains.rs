//! The module contains all of the utilities needed to handle Game Maker Studio reference chains.

use std::{collections::HashMap, io::Cursor};

use crate::data_loader::utils::{cursor::CustomCursor, error::DataLoadError};

/// This stores either a variable or function reference.
#[derive(Debug)]
pub struct Reference {
    /// The unique identifier of the variable or function the reference points to.
    pub id: u32,
    /// The type of the variable or function the reference points to.
    pub ref_type: u32,
}

/// This is a mask used where the masked bits is the relative location of the next occurance
/// and the unmasked bits are a byte corresponding to the type of the reference.
const REFERENCE_LOCATION_MASK: u32 = 0x07FFFFFF;

/// This resolves reference chains for both variables and functions
/// # Arguments
/// - `cursor`: The cursor which conatins the file data.
/// - `occurrences`: The number of occurrences in the reference chain.
/// - `id`: The identifier of the object referenced.
/// - `first_occurrence`: The first occurance of the referenced object.
/// - `reference_map`: The reference map to edit.
/// # Format Documentation.
/// In bytecode, it appears that rather than storing identifiers to the variable or function one wishes to
/// manipulate or call, the raw bytecode instead contains a relative pointer to the next occurance of that
/// variable or function along with type information about the reference. The reference chain is "resolved"
/// when, in the VARI and FUNC chunks, the variable or function is declared and its number of occurances and
/// first occurance address are resolved.
/// # Implementation Notes
/// Most likely in the original C++ runner the memory was changed directly, however since this implementation
/// is in Rust that is much more difficult. Namely because [`crate::data_loader::utils::cursor::Deserializable`]
/// is contextless and The cursor's internal type would need to be mutable, which would be a large and unneeded
/// change.
///
/// In order to handle that issue this function instead edits a map with the indices of the each reference and the
/// Reference object which it points to.
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
