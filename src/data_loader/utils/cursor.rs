//! This module handles all utilities regarding cursor handling and data reading.

use log::{info, warn};
use snafu::ResultExt;
use std::collections::HashMap;
use std::io::{Cursor, Read};

use crate::data_loader::utils::error::StringDecodeSnafu;
use crate::data_loader::utils::error::{DataLoadError, IOSnafu};

/// A custom trait for Cursor which allows for easier loading of typed data
/// # Examples
/// ```
/// // cursor is already defined as type T where T implements CustomCursor
/// let number = cursor.read_i32()?;
/// ```
pub trait CustomCursor {
    /// Reads 4 bytes at the cursor position sequentially into an array.
    /// # Examples
    /// ```
    /// // This can be seen in many of the (....)Chunk::deserialize functions
    ///
    /// let ident = cursor.read_ident()?;
    /// if ident != Chunk::IDENT {
    ///    return Err(DataLoadError::UnexpectedIdent {
    ///        pos: cursor.position() - 4,
    ///        expected: Chunk::IDENT,
    ///        actual: ident,
    ///    });
    /// }
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `4`, hence the `cursor.position() - 4` in the example.
    fn read_ident(&mut self) -> Result<[u8; 4], DataLoadError>;

    /// Reads a C-String at the cursor position.
    /// # Examples
    /// ```
    /// // This is from the implementation of Deserializable for String
    ///
    /// fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    /// where
    ///     Self: Sized,
    /// {
    ///     cursor.read_string()
    /// }
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by the length of the C-String.
    /// # Notes
    /// - The cursor should continue reading until it reaches the null terminator (`'\0'`).
    fn read_string(&mut self) -> Result<String, DataLoadError>;

    /// Reads 8 bytes at the cursor position as a little-endian unsigned 64-bit integer.
    /// # Examples
    /// ```
    /// let timestamp = cursor.read_u64()?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `8`.
    fn read_u64(&mut self) -> Result<u64, DataLoadError>;

    /// Reads 4 bytes at the cursor position as a little-endian unsigned 32-bit integer.
    /// # Examples
    /// ```
    /// let size = cursor.read_u32()?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `4`.
    fn read_u32(&mut self) -> Result<u32, DataLoadError>;

    /// Reads 2 bytes at the cursor position as a little-endian unsigned 16-bit integer.
    /// # Examples
    /// ```
    /// let locals_count = cursor.read_u16()?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `2`.
    fn read_u16(&mut self) -> Result<u16, DataLoadError>;

    /// Reads 1 byte at the cursor position as an unsigned 8-bit integer.
    /// # Examples
    /// ```
    /// let char_set = cursor.read_u8()?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `1`.
    fn read_u8(&mut self) -> Result<u8, DataLoadError>;

    /// Reads 4 bytes at the cursor position as a little-endian signed 32-bit integer.
    /// # Examples
    /// ```
    /// let x = cursor.read_i32()?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `4`.
    fn read_i32(&mut self) -> Result<i32, DataLoadError>;

    /// Reads 4 bytes at the cursor position as a single-precision floating point number.
    /// # Examples
    /// ```
    /// let scale_x = cursor.read_f32()?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `4`.
    fn read_f32(&mut self) -> Result<f32, DataLoadError>;

    /// Reads 1 byte at the cursor position as a boolean.
    /// # Examples
    /// ```
    /// let disable_debug = cursor.read_boolean()?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `1`.
    /// # Notes
    /// - This acts by reading in a [`u8`] and returning true if the value is not equal to 0 and false otherwise.
    fn read_boolean(&mut self) -> Result<bool, DataLoadError>;

    /// Reads 4 bytes at the cursor position as a boolean.
    /// # Examples
    /// ```
    /// let bold = cursor.read_wide_boolean()?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `4`.
    /// # Notes
    /// - This acts by reading in a [`u32`] and returning true if the value is not equal to 0 and false otherwise.
    /// - Game Maker Studio tends to prefer this format of boolean over regular booleans.
    fn read_wide_boolean(&mut self) -> Result<bool, DataLoadError>;

    /// Reads 4 bytes as an absolute address and moves the cursor there with and offset.
    /// Deserializes the object at the location and returns to the original location.
    /// # Examples
    /// ```
    /// let name = cursor.read_obj_pointer::<String>(0)?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `4`. This occurs even if there was an error deserializing the object.
    /// # Notes
    /// - The function will return an [`DataLoadError::InvalidReadError`] if the location passed in is less than or equal to 0.
    fn read_obj_pointer<T>(&mut self, offset: u32) -> Result<T, DataLoadError>
    where
        T: Deserializable;

    /// Reads 4 bytes as an absolute address and moves the cursor there with and offset.
    /// Deserializes the object at the location and returns to the original location.
    /// If the location is 0 or negative, `Ok(None)` is returned instead of an error.
    /// # Examples
    /// ```
    /// let action_name = cursor.read_opt_pointer::<String>(0)?;
    /// ```
    /// # Side-effects
    /// Increments `cursor.position()` by `4`. This occurs even if there was an error deserializing the object.
    fn read_opt_pointer<T>(&mut self, offset: u32) -> Result<Option<T>, DataLoadError>
    where
        T: Deserializable;

    /// Reads a list of pointers to objects and returns a map structure where the keys are the addresses to each object.
    /// # Examples
    /// ```
    /// let string_map = cursor.read_pointer_map::<String>(4)?;
    /// ```
    /// # Side-effects
    /// If execution is sucessful, the cursor will be located at the end of the pointer list.
    /// If execution is unsuccessful, the cursor will be at and undefined location.
    /// # Notes
    /// - The deseralization of each object carries the same implementation as [`CustomCursor::read_obj_pointer`], meaning that an offset can be passed.
    fn read_pointer_map<T>(&mut self, offset: u32) -> Result<HashMap<u32, T>, DataLoadError>
    where
        T: Deserializable;

    /// Reads a list of pointers to objects and returns a vector of objects.
    /// # Examples
    /// ```
    /// let code_entries = cursor.read_pointer_list::<CodeEntry>(0)?;
    /// ```
    /// # Side-effects
    /// If execution is sucessful, the cursor will be located at the end of the pointer list.
    /// If execution is unsuccessful, the cursor will be at and undefined location.
    /// # Notes
    /// - The deseralization of each object carries the same implementation as [`CustomCursor::read_obj_pointer`], meaning that an offset can be passed.
    fn read_pointer_list<T>(&mut self, offset: u32) -> Result<Vec<T>, DataLoadError>
    where
        T: Deserializable;
}

impl CustomCursor for Cursor<&[u8]> {
    fn read_ident(&mut self) -> Result<[u8; 4], DataLoadError> {
        let start_pos = self.position();
        let mut ident = [0u8; 4];
        self.read_exact(&mut ident)
            .context(IOSnafu { pos: start_pos })?;
        Ok(ident)
    }

    fn read_string(&mut self) -> Result<String, DataLoadError> {
        let start_pos = self.position();
        let mut name_bytes = Vec::new();
        loop {
            let mut b = [0u8; 1];
            self.read_exact(&mut b)
                .context(IOSnafu { pos: start_pos })?;
            if b[0] == 0 {
                break;
            }
            name_bytes.push(b[0]);
        }

        String::from_utf8(name_bytes).context(StringDecodeSnafu { pos: start_pos })
    }

    fn read_u64(&mut self) -> Result<u64, DataLoadError> {
        let start_pos = self.position();
        let mut bytes = [0u8; 8];
        self.read_exact(&mut bytes)
            .context(IOSnafu { pos: start_pos })?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn read_u32(&mut self) -> Result<u32, DataLoadError> {
        let start_pos = self.position();
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)
            .context(IOSnafu { pos: start_pos })?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_u16(&mut self) -> Result<u16, DataLoadError> {
        let start_pos = self.position();
        let mut bytes = [0u8; 2];
        self.read_exact(&mut bytes)
            .context(IOSnafu { pos: start_pos })?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn read_u8(&mut self) -> Result<u8, DataLoadError> {
        let start_pos = self.position();
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)
            .context(IOSnafu { pos: start_pos })?;
        Ok(bytes[0])
    }

    fn read_f32(&mut self) -> Result<f32, DataLoadError> {
        let start_pos = self.position();
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)
            .context(IOSnafu { pos: start_pos })?;
        Ok(f32::from_le_bytes(bytes))
    }

    fn read_i32(&mut self) -> Result<i32, DataLoadError> {
        let start_pos = self.position();
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)
            .context(IOSnafu { pos: start_pos })?;
        Ok(i32::from_le_bytes(bytes))
    }

    fn read_boolean(&mut self) -> Result<bool, DataLoadError> {
        let start_pos = self.position();
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)
            .context(IOSnafu { pos: start_pos })?;
        Ok(bytes[0] != 0)
    }

    fn read_wide_boolean(&mut self) -> Result<bool, DataLoadError> {
        Ok(self.read_u32()? != 0)
    }

    fn read_obj_pointer<T>(&mut self, offset: u32) -> Result<T, DataLoadError>
    where
        T: Deserializable,
    {
        let location = self.read_u32()?;
        if location == 0 {
            return Err(DataLoadError::InvalidReadError {
                pos: location as u64,
            });
        }
        let reset_pos = self.position();

        self.set_position((location + offset) as u64);

        let output = T::deserialize(self);

        self.set_position(reset_pos);

        output
    }

    fn read_opt_pointer<T>(&mut self, offset: u32) -> Result<Option<T>, DataLoadError>
    where
        T: Deserializable,
    {
        let location = self.read_u32()?;
        if location == 0 {
            return Ok(None);
        }

        self.set_position(self.position() - 4);

        Ok(Some(self.read_obj_pointer(offset)?))
    }

    fn read_pointer_map<T>(&mut self, offset: u32) -> Result<HashMap<u32, T>, DataLoadError>
    where
        T: Deserializable,
    {
        let number = self.read_u32()?;

        let mut output = HashMap::new();

        for _ in 0..number {
            let key = self.read_u32()?;
            self.set_position(self.position() - 4);
            output.insert(key, self.read_obj_pointer(offset)?);
        }

        Ok(output)
    }

    fn read_pointer_list<T>(&mut self, offset: u32) -> Result<Vec<T>, DataLoadError>
    where
        T: Deserializable,
    {
        let number = self.read_u32()?;

        let mut output = Vec::new();
        for _ in 0..number {
            output.push(self.read_obj_pointer(offset)?);
        }

        Ok(output)
    }
}

/// Indicates an object which can be deserialized without external information.
pub trait Deserializable {
    /// Takes in a cursor at the start of the object location and returns the deserialized object
    /// # Side Effects
    /// [`Deserializable::deserialize`] is allowed to affect the cursor location
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized;
}

impl Deserializable for String {
    /// Reads a C-String using [`CustomCursor::read_string`].
    /// Notes:
    /// - This is here for use with [`CustomCursor::read_obj_pointer`] and similar functions.
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        cursor.read_string()
    }
}

impl<T> Deserializable for Vec<T>
where
    T: Deserializable,
{
    /// Reads a pointer list using [`CustomCursor::read_pointer_list`].
    /// Notes:
    /// - This is here for use with [`CustomCursor::read_obj_pointer`] and similar functions.
    /// - Game Maker Studio does also have lists with data sequentially, which has to be handled in other ways.
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        cursor.read_pointer_list::<T>(0)
    }
}

/// Moves the cursor to the expected end of a chunk and provides logging information.
/// # Arguments
/// - `cursor`: The cursor which will be moved.
/// - `start_pos`: The starting location of the chunk.
/// - `size`: The size of the chunk.
/// - `exists_map`: Whether or not there was a pointer map or list in the chunk.
/// # Notes
/// - The additional logging information is given based on the cursor location.
///   - If the cursor is outside of the chunk then the function will return an error.
///   - If the cursor is within the chunk but within 16 bytes of the end it could be either that the chunk
///     not fully read or that there is padding.
///   - If the cursor is within the chunk but not within 16 bytes of the end if there was a map it could be
///     that the cursor is misaligned. If there was not then it is definitely a bug.
pub fn handle_cursor_alignment(
    cursor: &mut Cursor<&[u8]>,
    start_pos: u64,
    size: u64,
    exists_map: bool,
) -> Result<(), DataLoadError> {
    let expected_end = start_pos + size;
    if cursor.position() > expected_end {
        return Err(DataLoadError::ChunkEscapeError {
            pos: cursor.position(),
            loc: expected_end,
        });
    } else if cursor.position() < expected_end {
        if expected_end - cursor.position() < 16 || exists_map {
            info!(
                "Cursor position at {}, moving to {}. (This could be chunk alignment, a map, or a bug)",
                cursor.position(),
                expected_end
            );
        } else {
            warn!(
                "Cursor position at {}, moving to {}. (This is a bug)",
                cursor.position(),
                expected_end
            );
        }
        cursor.set_position(expected_end);
    }

    Ok(())
}
