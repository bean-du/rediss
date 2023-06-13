use crate::Result;
use bytes::BytesMut;
use std::io::Cursor;
use tokio::{
    io::{AsyncReadExt, BufWriter},
    net::TcpStream,
};

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

    pub async fn read_packet(&mut self) -> crate::Result<Option<Packet>> {
        loop {
            // Attempt to parse a frame from the buffered data. If enough data
            // has been buffered, the frame is returned.
            if let Some(frame) = self.parse_packet().await? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    pub async fn parse_packet(&mut self) -> Result<Option<Packet>> {
        let mut buf = Cursor::new(self.buffer.to_vec());

        let packet = Packet::parse(&mut buf).await?;

        Ok(Some(packet))
    }

    pub async fn write_packet(&self, pkt: Packet) -> std::io::Result<()> {
        Ok(())
    }
}
