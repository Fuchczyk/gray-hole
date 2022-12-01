use std::mem::size_of;

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};

pub trait MessageWrite<M>
where
    Self: std::io::Write,
    M: Serialize,
{
    fn write_message(&mut self, message: &M) -> Result<usize>;
}

impl<T: std::io::Write, M: Serialize> MessageWrite<M> for T {
    fn write_message(&mut self, message: &M) -> Result<usize> {
        let serialized_message =
            bincode::serialize(message).context("Problem with serialize implementation.")?;

        let serialized_message_size = serialized_message.len().to_be_bytes();

        self.write_all(&serialized_message_size)
            .context("Unable to write message to writer.")?;
        self.write_all(&serialized_message)
            .context("Unable to write message to writer.")?;

        Ok(serialized_message_size.len() + serialized_message.len())
    }
}

pub trait MessageRead<M>
where
    Self: std::io::Read,
    M: DeserializeOwned,
{
    fn read_message(&mut self) -> Result<M>;
}

impl<T: std::io::Read, M: DeserializeOwned> MessageRead<M> for T {
    fn read_message(&mut self) -> Result<M> {
        const USIZE_LEN: usize = size_of::<usize>();

        let mut message_size = [0; USIZE_LEN];
        self.read_exact(&mut message_size)
            .context("Unable to read message from reader.")?;

        let message_size = usize::from_be_bytes(message_size);

        let mut message_buffer = vec![0 ; message_size];
        
        eprintln!("DESERIALIZING [{}] OF SIZE [{}]", std::any::type_name::<T>(), message_size);

        self.read_exact(&mut message_buffer)
            .context("Unable to read message from reader.")?;

        bincode::deserialize(&message_buffer).context("Problem with deserialize implementation.")
    }
}
