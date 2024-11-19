use std::collections::HashMap;

use crate::FILE_DATA;

#[macro_export]
macro_rules! read_adv {
    ( $t:ty, $x:expr, $y:ident ) => {
        {
            let out = <$t>::from_le_bytes($x[$y..($y + size_of::<$t>())].try_into().expect(""));
            $y += size_of::<$t>();
            out
        }
    };
}

#[macro_export]
macro_rules! read_num {
    ( $t:ty, $x:expr, $y:expr ) => {
        {
            <$t>::from_le_bytes($x[$y..($y + size_of::<$t>())].try_into().expect(""))
        }
    };
}

pub trait Deserializable {
    fn deserialize(pointer: usize) -> Self;
}

pub fn read_pointer_map<T>(p: usize, offset: usize) -> HashMap<u32, T>
where 
    T: Deserializable
{
    let mut pointer = p;
    let slice = FILE_DATA.as_slice();
    let len = read_adv!(u32, slice, pointer);
    let mut map = HashMap::new();

    for _ in 0..len {
        map.insert((pointer + offset) as u32, T::deserialize(read_adv!(u32, slice, pointer) as usize));
    }

    map
}