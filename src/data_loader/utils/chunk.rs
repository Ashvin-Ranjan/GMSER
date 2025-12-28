use crate::data_loader::utils::cursor::Deserializable;

pub trait Chunk
where
    Self: Deserializable,
{
    const IDENT: [u8; 4];
}
