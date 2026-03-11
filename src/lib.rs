mod communication_backend;
mod mailbox;

use std::{
    error::Error,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use crate::{
    communication_backend::CommunicationBackend,
    mailbox::{Envelope, Sender},
};

/// A `ActorSystem` controls actors within the system lifetimes. Actors in the system is spawned
/// and killed here. When the `ActorSystem` is dropped, all actors in the system are killed.
pub struct ActorSystem {
    counter: u64, // ref_registry: HashMap<String, &d>,
}

impl ActorSystem {
    /// This spawns an actor which runs on tokio thread and returns [ActorRef] corresponding to
    /// the spawned actor.
    pub async fn spawn<A: Actor, B: CommunicationBackend>(
        &mut self,
        initial_state: A::State,
    ) -> ActorRef<A, B> {
        let (tx, rx) = B::message_channel::<A>();
        let actor_id = ActorId::LocalId(self.counter);
        let handle = tokio::spawn(async move {
            let actor = A::new(initial_state);
            loop {}
        });
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

/// `ActorRef` contains actor information and a mean to send message to the corresponding actor.
pub struct ActorRef<A, B>
where
    A: Actor,
    B: CommunicationBackend,
{
    id: ActorId,
    tx: B::Sender<A>,
    _marker: PhantomData<A>,
}

pub struct ActorContext {
    id: ActorId,
    name: String,
    system_ref: Arc<RwLock<ActorSystem>>,
    // created_at
}

impl ActorContext {
    fn new(name: String, system: Arc<RwLock<ActorSystem>>) -> Self {
        todo!()
    }
}

pub enum ActorError {}

pub trait Actor: Send + Sync {
    type State: Send + Sync;
    type Message: Send + Sync;
    type Response: Send + Sync;

    fn new(initial_state: Self::State) -> Self;

    /// The `handle` method tells how the actor processes the given message, and produces response.
    async fn handle(
        &mut self,
        message: Self::Message,
        ctx: &ActorContext,
    ) -> Result<Self::Response, ActorError>;
}

impl<A: Actor, B: CommunicationBackend> ActorRef<A, B> {
    /// This function sends a message of to the actor corresponding to the [ActorRef].
    async fn send(&mut self, msg: A::Message) -> Result<(), ActorRefError> {
        // TODO: If send by the actor handler, the envelop should include the sender information
        let envelope = Envelope::new(msg);
        Sender::send(&mut self.tx, envelope)
            .await
            // XXX: How to handle error?
            .map_err(|e| ActorRefError::SendError(e))
    }
}
