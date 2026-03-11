use std::{error::Error, fmt::Display};

use crate::{Actor, ActorId};

/// A wrapper around the message containing metadata, e.g., sender information.
pub struct Envelope<T: Actor> {
    /// A message wrapped in the envelope.
    pub message: T::Message,
    /// If the message is sent from within an actor handler, the `sender` will contain the
    /// [ActorId] of the sender.
    pub sender: Option<ActorId>,
    /// Some command needs response from the actor, this channel sends back the response after the
    /// actor processes the message.
    pub reply_channel: Option<tokio::sync::oneshot::Sender<T::Response>>,
}

impl<T: Actor> Envelope<T> {
    pub fn new(message: T::Message) -> Self {
        Self {
            message,
            sender: None,
            reply_channel: None,
        }
    }
}

#[derive(Debug)]
enum MailboxError {}

impl Display for MailboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for MailboxError {}

/// A wrapper trait for mailbox sender.
pub trait Sender<T: Actor> {
    async fn send(&self, value: Envelope<T>) -> Result<(), MailboxError>;
}

impl<T: Actor> Sender<T> for tokio::sync::mpsc::Sender<Envelope<T>> {
    async fn send(&self, value: Envelope<T>) -> Result<(), MailboxError> {
        self.send(value).await.map_err(|e| todo!())
    }
}

/// A wrapper trait for mailbox receiver.
pub trait Receiver<T: Actor> {
    async fn recv(&self, value: Envelope<T>) -> Result<(), MailboxError>;
}

impl<T: Actor> Receiver<T> for tokio::sync::mpsc::Receiver<Envelope<T>> {
    async fn recv(&self, value: Envelope<T>) -> Result<(), MailboxError> {
        todo!()
    }
}
