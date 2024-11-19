use crate::{read_num, utils::read::Deserializable, FILE_DATA};

use std::str;

impl Deserializable for String {
    fn deserialize(pointer: usize) -> Self {
        let len = read_num!(u32, FILE_DATA.as_slice(), pointer);
        return str::from_utf8(&FILE_DATA.as_slice()[pointer+4..(pointer+4+len as usize)]).expect("Unable to parse string").to_owned();
    }
}