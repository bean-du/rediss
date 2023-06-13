use crate::Result;
use std::fmt;
use std::io::Cursor;
use bytes::{Buf};
use tokio::io::AsyncReadExt;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    Incomplete,

    /// Data megic error
    MegicIncorrect,

    /// Invalid message encoding
    Other(crate::Error),
}

#[derive(Debug, Default, Clone)]
pub struct Packet {
    pub megic: u8,
    pub len: u8,
    pub id: u8,
    pub cmd: u8,
    pub data: Vec<u8>,
}

impl Packet {
    pub async fn parse(src: &mut Cursor<Vec<u8>>) -> Result<Packet> {
        let megic = get_u8(src).await?;
        if megic != 0xfe {
           return  Err(Box::new(Error::MegicIncorrect))
        }
        let len = get_u8(src).await?;
        let id = get_u8(src).await?;
        let cmd = get_u8(src).await?;
        let body_len = len as usize - 2;
        let mut body = vec![0u8; body_len];
        src.read_exact(&mut body).await?;

        Ok(Packet{
            megic,
            len,
            id,
            cmd,
            data: body
        })
    }

    pub fn pack() -> Result<u8> {
        
        Ok(())
}

async fn  get_u8(src: &mut Cursor<Vec<u8>>) -> Result<u8> {
    if !src.has_remaining() {
        return Err(Box::new(Error::Incomplete))
    }
    match src.read_u8().await {
        Ok(u) => Ok(u),
        Err(e) => Err(Box::new(e))
    }
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


impl From<String> for Error {
    fn from(src: String) -> Error {
        Error::Other(src.into())
    }
}

impl From<&str> for Error {
    fn from(src: &str) -> Error {
        src.to_string().into()
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_src: FromUtf8Error) -> Error {
        "protocol error; invalid packet format".into()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid packet format".into()
    }
}

impl std::error::Error for Error {}


