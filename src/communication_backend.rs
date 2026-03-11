use crate::{
    Actor,
    mailbox::{Envelope, Receiver as MailReceiver, Sender as MailSender},
};

/// An abstract grouping the communication in/out actors.
pub trait CommunicationBackend {
    type Sender<T: Actor>: MailSender<T>;
    type Receiver<T: Actor>: MailReceiver<T>;

    /// A method to spawn `tx` and `rx` for communicating with the actor.
    fn message_channel<T: Actor>() -> (Self::Sender<T>, Self::Receiver<T>);
}

pub struct TokioBackend;

impl CommunicationBackend for TokioBackend {
    type Sender<T: Actor> = tokio::sync::mpsc::Sender<Envelope<T>>;

    type Receiver<T: Actor> = tokio::sync::mpsc::Receiver<Envelope<T>>;

    fn message_channel<T: Actor>() -> (Self::Sender<T>, Self::Receiver<T>) {
        /// TODO: Make size configurable
        const SIZE: usize = u16::MAX as usize;
        tokio::sync::mpsc::channel::<Envelope<T>>(SIZE)
    }
}
