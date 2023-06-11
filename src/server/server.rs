use crate::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{self, Duration};
use tracing::{debug, error, info, instrument};

use super::connection::{self, Connection};

#[derive(Debug)]
pub struct Listener {
    listener: TcpListener,
}

struct Hadnler {
    connection: Connection,
}

impl Listener {
    async fn run(&mut self) -> Result<()> {
        info!("accepting inbound connections");

        loop {
            let socket= self.accept().await?;

            let handle = Hadnler::new(Connection::new(socket));

            tokio::spawn(async move {
                if let Err(e) = handle.run().await {
                    error!(cause = ?e, "connection error");
                }
            });
        }
    }

    async fn accept(&mut self) -> Result<TcpStream> {
        let mut backoff = 1;

        // Try to accept a few times
        loop {
            // Perform the accept operation. If a socket is successfully
            // accepted, return it. Otherwise, save the error.
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    if backoff > 64 {
                        // Accept has failed too many times. Return the error.
                        return Err(err.into());
                    }
                }
            }

            // Pause execution until the back off period elapses.
            time::sleep(Duration::from_secs(backoff)).await;

            // Double the back off
            backoff *= 2;
        }
    }
}

impl Hadnler {
    pub fn new(connection: Connection) -> Self {
        Hadnler { connection }
    }

    async fn run(&self) -> Result<()> {
        // read packet from connection

        // handle packet

        // response to client

        Ok(())
    }
}
