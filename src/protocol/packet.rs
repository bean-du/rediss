use crate::Result;
use std::fmt;
use std::io::Cursor;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    Incomplete,

    /// Data megic error
    MegicIncorrect,

    /// Invalid message encoding
    Other(crate::Error),
}

#[derive(Debug, Default)]
pub struct Packet {
    pub magic: u8,
    pub len: u8,
    pub id: u8,
    pub cmd: u8,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Packet> {
        let megic = get_u8(src)?;
        if megic != 0xfe {
            Err(Error::MegicIncorrect)
        }
        let len = get_u8(src)?;
        let id = get_u8(src)?;
        let cmd = get_u8(src)?;
        let body_len = len as usize - 2;
        let mut body = vec![0u8; body_len];
        src.read_exact(body)?;

        Ok(Packet::default())
    }
}
fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8> {
    if !src.has_remaining() {
        Err(Error::Incomplete)
    }

    Ok(src.read_u8())
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => "stream ended early".fmt(fmt),
            Error::MegicIncorrect => "megic incorrect".fmt(fmt),
            Error::Other(err) => err.fmt(fmt),
        }
    }
}
