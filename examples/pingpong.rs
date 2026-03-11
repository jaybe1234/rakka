use anyhow::Result;

struct PingPong;

struct Ping;
struct Pong;

impl Actor for Ping {
    type Message = Ping;
    type Response = Pong;

    pub fn handle(&mut self, msg: Envelope<Self::Message>) -> impl Future<Output = Self::Response> {
        async { Ok(Pong) }
    }

    fn send(&mut self, msg: Envelope<Self::Message>) -> impl Future<Output = Result<()>> {
        async move { Ok(()) }
    }
}

async fn main() {
    let system = ActorSystem::default();
    let (mut ping, mut ping_ref) = system.spawn::<Ping>();

    let pong = ping_ref.send(Pong).await?;
}
