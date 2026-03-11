use std::{any::Any, collections::HashMap, error::Error};

use anyhow::Result;

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

/// A `ActorSystem` controls actors within the system lifetimes. Actors in the system is spawned
/// and killed here. When the `ActorSystem` is dropped, all actors in the system are killed.
pub struct ActorSystem {
    // ref_registry: HashMap<String, &d>,
}

impl ActorSystem {
    /// This spawns an actor which runs on tokio thread and returns [ActorRef] corresponding to
    /// the spawned actor.
    pub async fn spawn<T: Actor>(initial_state: T::State) -> (ActorRef<T>) {
        let actor = T::new(initial_state);
        todo!();
    }
    // TODO: Supervisor patterns.
}

/// `ActorId` this the unique identifier of an actor.
pub enum ActorId {
    LocalId(u64),
    // Remote
}

#[derive(Debug)]
enum ActorRefError {
    SendError(std::io::Error),
}

impl Error for ActorRefError {}

impl std::fmt::Display for ActorRefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// `ActorRef` contains mean to control the referred actor, such as, send a message, kill, and etc.
pub struct ActorRef<T: Actor> {
    id: ActorId,
    tx: tokio::sync::mpsc::Sender<Envelope<T>>,
}

impl<T: Actor> ActorRef<T> {
    /// This function sends a message of to the actor corresponding to the [ActorRef].
    async fn send(&mut self, msg: T::Message) -> Result<(), ActorRefError> {
        // TODO: If send by the actor handler, the envelop should include the sender information
        let envelope = Envelope::new(msg);
        self.tx
            .send(envelope)
            .await
            // XXX: How to properly parse [tokio::sync::mpsc::error::SendError] to a wrapper?
            .map_err(|e| ActorRefError::SendError(e.into()))
    }
}

pub trait Actor: Send + Sync {
    type State: Send + Sync;
    type Message: Send + Sync;
    type Response: Send + Sync;

    fn new(initial_state: Self::State) -> Self;
}
