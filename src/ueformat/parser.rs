use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use zstd::Decoder;
use crate::ueformat::error::{ParseError};


pub struct UEFileParser {
    cursor: Cursor<Vec<u8>>,
    size: usize,
}

impl UEFileParser {
    pub fn new(data: Vec<u8>) -> Self {
        let len = data.len();
        Self { cursor: Cursor::new(data), size: len }
    }

    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, ParseError> {
        let mut buf = vec![0u8; size];
        match self.cursor.read_exact(&mut buf) {
            Ok(_) => Ok(buf),
            Err(cur_err) => Err(ParseError::CursorError(cur_err))
        }
    }

    pub fn read_bool(&mut self) -> Result<bool, ParseError> {
        match self.cursor.read_u8() {
            Ok(b) => Ok(b != 0),
            Err(cur_err) => Err(ParseError::CursorError(cur_err))
        }
    }

    pub fn read_byte(&mut self) -> Result<u8, ParseError> {
        match self.cursor.read_u8() {
            Ok(b) => Ok(b),
            Err(cur_err) => Err(ParseError::CursorError(cur_err))
        }
    }

    pub fn read_string(&mut self, size: usize) -> Result<String, ParseError> {
        match self.read(size) {
            Ok(bytes) => {
                Ok(String::from_utf8_lossy(&bytes).into_owned())
            },
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn read_int(&mut self) -> Result<i32, ParseError> {
        match self.cursor.read_i32::<LittleEndian>() {
            Ok(i32) => Ok(i32),
            Err(cur_err) => Err(ParseError::CursorError(cur_err))
        }
    }

    pub fn read_fstring(&mut self) -> Result<String, ParseError> {
        let len = match self.cursor.read_i32::<LittleEndian>() {
            Ok(i32) => i32 as isize,
            Err(err) => return Err(ParseError::CursorError(err))
        };
        if len < 0 {
            return Ok(String::new());
        }
        let len = len as usize;
        match self.read_string(len) {
            Ok(str) => Ok(str),
            Err(err) => Err(err)
        }
    }

    pub fn read_float_vector(&mut self, size: usize) -> Result<Vec<f32>, ParseError> {
        let mut out = Vec::with_capacity(size);
        for _ in 0..size {
            out.push(
                match self.cursor.read_f32::<LittleEndian>() {
                    Ok(d) => d,
                    Err(cur_err) => return Err(ParseError::CursorError(cur_err))
                }
            );
        }
        Ok(out)
    }

    pub fn read_int_vector(&mut self, size: usize) -> Result<Vec<u32>, ParseError> {
        if size == 0 {
            return Ok(Vec::new());
        }
        let mut out = Vec::with_capacity(size);
        for _ in 0..size {
            out.push(
                match self.cursor.read_u32::<LittleEndian>() {
                    Ok(d) => d,
                    Err(cur_err) => return Err(ParseError::CursorError(cur_err))
                }
            );
        }
        Ok(out)
    }

    pub fn decompress_remaining_to_vec(&mut self) -> Result<Vec<u8>, ParseError> {
        let pos = self.cursor.position() as usize;
        let all = self.cursor.get_ref();
        let slice = &all[pos..];

        let cursor = Cursor::new(slice);
        let mut dec = Decoder::new(cursor)
            .map_err(|e| ParseError::CursorError(e)).unwrap();
        let mut out = Vec::new();
        match dec.read_to_end(&mut out) {
            Ok(_) => Ok(out),
            Err(cur_err) => Err(ParseError::CursorError(cur_err))
        }
    }

    pub fn read_to_end(&mut self) -> Result<Vec<u8>, ParseError> {
        let pos = self.cursor.position() as usize;
        let all = self.cursor.get_ref();
        if pos >= all.len() {
            return Ok(Vec::new());
        }
        Ok(all[pos..].to_vec())
    }

    pub fn eof(&self) -> bool {
        self.cursor.position() as usize >= self.size
    }

    pub fn skip(&mut self, offset: i64) -> Result<(), ParseError> {
        match self.cursor.seek(SeekFrom::Current(offset)) {
            Ok(_) => Ok(()),
            Err(cur_err) => Err(ParseError::CursorError(cur_err))
        }
    }

    pub fn goto(&mut self, pos: u64) -> Result<(), ParseError> {
        match self.cursor.seek(SeekFrom::Start(pos)) {
            Ok(_) => Ok(()),
            Err(cur_err) => Err(ParseError::CursorError(cur_err))
        }
    }

    pub fn get_pos(&self) -> u64 {
        self.cursor.position()
    }

    pub fn override_size(&mut self, new_size: usize) {
        self.size = new_size;
    }
}

