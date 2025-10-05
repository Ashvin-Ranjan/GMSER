use std::io::{self, Read};

use snafu::ResultExt;

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

        return String::from_utf8(name_bytes).context(StringDecodeSnafu { pos: start_pos });
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
        let reset_pos = self.position();

        self.set_position((location + offset) as u64);

        let output = callback(self);

        self.set_position(reset_pos);

        output
    }
}

pub fn read_string_callback(cursor: &mut io::Cursor<&[u8]>) -> Result<String, DataLoadError> {
    cursor.read_string()
}
