use anyhow::{Context, Result};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};

use crate::{
    message::{MessageRead, MessageWrite},
    types::{ClientMessage, ServerMessage},
};

const MAX_CONNECTIONS: i32 = 32000;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    fn bind_listener(address: SocketAddr) -> Result<TcpListener> {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))
            .context("Unable to create socket.")?;

        socket
            .bind(&address.into())
            .context("Unable to bind socket.")?;

        socket
            .listen(MAX_CONNECTIONS)
            .context("Unable to set socket to listening.")?;

        Ok(socket.into())
    }

    pub fn new(ip: Ipv4Addr, port: u16) -> Result<Self> {
        let listener_address = SocketAddrV4::new(ip, port);

        let listener = Self::bind_listener(listener_address.into())?;

        Ok(Self { listener })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut client_buffer: Option<(TcpStream, SocketAddr)> = None;

        while let Ok((mut connection, address)) = self.listener.accept() {
            let message: ClientMessage = connection.read_message()?;
            eprintln!("GOT CONNECTION FROM {:?} WITH MESSAGE {message:?}", address);

            match client_buffer {
                None => client_buffer = {
                    eprintln!("Waiting for second.");
                    Some((connection, address))
                },
                Some((mut first_connection, first_address)) => {
                    eprintln!("Connecting");
                    let listen_message = ServerMessage::YouListen;
                    first_connection.write_message(&listen_message)?;

                    let client_message: ClientMessage = first_connection.read_message()?;
                    if client_message != ClientMessage::ReadyToListen {
                        panic!("Programming went wrong.");
                    }

                    let connect_message = ServerMessage::YouConnect {
                        address: first_address,
                    };
                    connection.write_message(&connect_message)?;

                    client_buffer = None;
                }
            }
        }

        Ok(())
    }
}
