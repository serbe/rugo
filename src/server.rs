use  actix::{Actor, Context, Handler, Message, Recipient};
// use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Msg(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Msg>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

pub struct WsServer {
    sessions: HashMap<usize, Recipient<Msg>>,
    ids: usize,
}

impl Default for WsServer {
    fn default() -> WsServer {
        WsServer {
            sessions: HashMap::new(),
            ids: 1,
        }
    }
}

impl Actor for WsServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room
        // self.send_message(&"Main".to_owned(), "Someone joined", 0);

        // register session with random id
        let id = self.ids;
        self.sessions.insert(id, msg.addr);
        self.ids += 1;

        id
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");
        // remove address
        self.sessions.remove(&msg.id);
    }
}
