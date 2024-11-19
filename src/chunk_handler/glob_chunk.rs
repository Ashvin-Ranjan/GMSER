use crate::{read_adv, FILE_DATA};

use super::chunk::Chunk;

pub fn read_global_entries(c: Chunk) -> Vec<u32> {
    let mut pointer = c.start as usize + 8;
    let slice = FILE_DATA.as_slice();
    let mut out = Vec::new();

    let len = read_adv!(u32, slice, pointer);

    for _ in 0..len {
        out.push(read_adv!(u32, slice, pointer));
    }

    out
}