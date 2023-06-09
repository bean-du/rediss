use std::io::Cursor;

use bytes::BytesMut;
use tokio::{io::BufWriter, net::TcpStream};

use crate::protocol::packet::Packet;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read(&mut self) -> Result<Packet> {
        let mut buf = Cursor::new(self.buffer[..]);
    }
}
