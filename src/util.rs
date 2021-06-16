
use std::io::Read;

use bytes::{Buf, BytesMut};

use crate::Error;

pub fn read_varuint<R: Read + ?Sized>(reader: &mut R) -> Result<(u32, usize), Error> {
    let mut decoded_value = 0;
    let mut raw_buffer = [0; 1];
    let mut next_byte = 0;

    for shift_amount in 0..5 {
        if reader.read(&mut raw_buffer)? == 1 {
            next_byte = raw_buffer[0];
        } else {
            Err("Unexpected EOF")?;
        }

        decoded_value |= ((next_byte & 0x7F) as u32) << shift_amount * 7;
        if next_byte & 0x80 == 0 {
            return Ok((decoded_value, shift_amount + 1));
        }
    }

    Err("Invalid VarInt")?
}

pub trait ReadExt: Read {
    fn read_u16_le(&mut self) -> Result<u16, Error> {
        let mut buf = BytesMut::from(&[0u8; 2][..]);
        self.read_exact(&mut buf)?;
        Ok(buf.get_u16_le())
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl<R: Read + ?Sized> ReadExt for R {}
