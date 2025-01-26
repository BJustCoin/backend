use std::collections::HashMap;

use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::{error::Result as SerdeResult, to_string, Value};

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(ActixMessage, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct MessageToClient {
    pub types: String,
    pub id:    i32,
    pub data:  Value,
}

impl MessageToClient {
    pub fn new(types: &str, id: i32, data: Value) -> Self {
        Self {
            types: types.to_string(),
            id:    id,
            data,
        }
    }
}

pub struct Server {
    sessions: HashMap<String, Recipient<Message>>
}

impl Server {
    pub fn new() -> Self {
        Server {
            sessions: HashMap::new(),
        }
    }

    fn send_message(&self, data: SerdeResult<String>) {
        match data {
            Ok(data) => {
                for recipient in self.sessions.values() {
                    match recipient.try_send(Message(data.clone())) {
                        Err(err) => {}
                        _ => {}
                    }
                }
            }
            Err(err) => {}
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub id: String,
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id.clone(), msg.addr);
    }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<MessageToClient> for Server {
    type Result = ();

    fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
        self.send_message(to_string(&msg));
    }
}
