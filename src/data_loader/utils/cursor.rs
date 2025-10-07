use log::{info, warn};
use snafu::ResultExt;
use std::collections::HashMap;
use std::io::{self, Read};

use crate::data_loader::utils::error::StringDecodeSnafu;
use crate::data_loader::utils::error::{DataLoadError, IOSnafu};

pub trait CustomCursor {
    fn read_ident(&mut self) -> Result<[u8; 4], DataLoadError>;

    fn read_string(&mut self) -> Result<String, DataLoadError>;

    fn read_u64(&mut self) -> Result<u64, DataLoadError>;
    fn read_u32(&mut self) -> Result<u32, DataLoadError>;
    fn read_u16(&mut self) -> Result<u16, DataLoadError>;
    fn read_u8(&mut self) -> Result<u8, DataLoadError>;
    fn read_f32(&mut self) -> Result<f32, DataLoadError>;

    fn read_boolean(&mut self) -> Result<bool, DataLoadError>;

    fn read_obj_pointer<T>(
        &mut self,
        callback: fn(&mut io::Cursor<&[u8]>) -> Result<T, DataLoadError>,
        offset: u32,
    ) -> Result<T, DataLoadError>;

    fn read_opt_pointer<T>(
        &mut self,
        callback: fn(&mut io::Cursor<&[u8]>) -> Result<T, DataLoadError>,
        offset: u32,
    ) -> Result<Option<T>, DataLoadError>;

    fn read_pointer_map<T>(
        &mut self,
        callback: fn(&mut io::Cursor<&[u8]>) -> Result<T, DataLoadError>,
        offset: u32,
    ) -> Result<HashMap<u32, T>, DataLoadError>;
}

impl CustomCursor for io::Cursor<&[u8]> {
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

    fn read_boolean(&mut self) -> Result<bool, DataLoadError> {
        let start_pos = self.position();
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)
            .context(IOSnafu { pos: start_pos })?;
        Ok(bytes[0] != 0)
    }

    fn read_obj_pointer<T>(
        &mut self,
        callback: fn(&mut io::Cursor<&[u8]>) -> Result<T, DataLoadError>,
        offset: u32,
    ) -> Result<T, DataLoadError> {
        let location = self.read_u32()?;
        if location == 0 {
            return Err(DataLoadError::InvalidReadError {
                pos: location as u64,
            });
        }
        let reset_pos = self.position();

        self.set_position((location + offset) as u64);

        let output = callback(self);

        self.set_position(reset_pos);

        output
    }

    fn read_opt_pointer<T>(
        &mut self,
        callback: fn(&mut io::Cursor<&[u8]>) -> Result<T, DataLoadError>,
        offset: u32,
    ) -> Result<Option<T>, DataLoadError> {
        let location = self.read_u32()?;
        if location == 0 {
            return Ok(None);
        }

        self.set_position(self.position() - 4);

        Ok(Some(self.read_obj_pointer(callback, offset)?))
    }

    fn read_pointer_map<T>(
        &mut self,
        callback: fn(&mut io::Cursor<&[u8]>) -> Result<T, DataLoadError>,
        offset: u32,
    ) -> Result<HashMap<u32, T>, DataLoadError> {
        let number = self.read_u32()?;

        let mut output = HashMap::new();

        for _ in 0..number {
            let key = self.read_u32()?;
            self.set_position(self.position() - 4);
            output.insert(key, self.read_obj_pointer(callback, offset)?);
        }

        Ok(output)
    }
}

pub fn read_string_callback(cursor: &mut io::Cursor<&[u8]>) -> Result<String, DataLoadError> {
    cursor.read_string()
}

pub fn handle_cursor_alignment(
    cursor: &mut io::Cursor<&[u8]>,
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
