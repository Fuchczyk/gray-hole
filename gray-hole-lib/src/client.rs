use std::net::{IpAddr, Ipv4Addr, TcpStream};
use std::net::{SocketAddr, TcpListener};

use anyhow::Context;
use anyhow::Result;
use socket2::{Domain, Protocol, Socket, Type};

use crate::message::{MessageRead, MessageWrite};
use crate::types::{ClientMessage, ConnectionMessage, ServerMessage};

#[derive(Debug)]
pub struct Client {
    connection: TcpStream,
}

const LISTENER_LIMIT: i32 = 4096;

fn connect_with_client(client_address: SocketAddr) -> Result<TcpStream> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))
        .context("Unable to build Socket structure.")?;

    socket
        .connect(&client_address.into())
        .context("Unable to connect socket to other client.")?;

    let stream = socket.into();

    Ok(stream)
}

fn connect_with_server(relay_ip: Ipv4Addr, relay_port: u16) -> Result<TcpStream> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))
        .context("Unable to build Socket structure.")?;

    socket
        .set_reuse_address(true)
        .context("Unable to set reusing address option.")?;

    let address = SocketAddr::new(IpAddr::V4(relay_ip), relay_port);

    socket
        .connect(&address.into())
        .context("Unable to connect socket to relay server.")?;

    socket.listen(LISTENER_LIMIT)
        .context("Unable to set listener limit.")?;

    let stream: TcpStream = socket.into();

    Ok(stream)
}

fn bind_listener(address: SocketAddr) -> Result<TcpListener> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))
        .context("Unable to build Socket structure.")?;

    socket
        .set_reuse_address(true)
        .context("Unable to set reusing address option.")?;

    socket
        .bind(&address.into())
        .context("Unable to bind socket to address.")?;

    let listener: TcpListener = socket.into();

    Ok(listener)
}

impl Client {
    pub fn transmit_data(&mut self, message: &[u8]) -> Result<()> {
        let message = ConnectionMessage::DataLoad {
            data: message.into(),
        };

        self.connection.write_message(&message)?;

        Ok(())
    }

    pub fn receive_data(&mut self) -> Result<Vec<u8>> {
        let message: ConnectionMessage = self.connection.read_message()?;

        let ConnectionMessage::DataLoad { data } = message;

        Ok(data)
    }

    pub fn new(relay_ip: Ipv4Addr, relay_port: u16) -> Result<Self> {
        let mut relay_connection = connect_with_server(relay_ip, relay_port)?;
        let connect_message = ClientMessage::Ready;

        relay_connection.write_message(&connect_message)?;

        let message: ServerMessage = relay_connection.read_message()?;

        match message {
            ServerMessage::YouConnect { address } => {
                let client_stream = connect_with_client(address)?;

                Ok(Self {
                    connection: client_stream,
                })
            }
            ServerMessage::YouListen => {
                let local_address = relay_connection
                    .local_addr()
                    .context("Unable to read address from stream.")?;

                let listener = bind_listener(local_address).context("Unable to bind listener.")?;

                let listen_message = ClientMessage::ReadyToListen;
                relay_connection.write_message(&listen_message)?;

                let (client_connection, _) = listener
                    .accept()
                    .context("Unable to retrieve connection from other client")?;

                Ok(Self {
                    connection: client_connection,
                })
            }
        }
    }
}
