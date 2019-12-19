use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use ws::{Handler, Handshake, Message, Sender, Result as WSResult};

use crate::error::Result;
use crate::db::ClientMessage;

pub struct Server {
    pub out: Sender,
    pub pool: Pool<PostgresConnectionManager>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> WSResult<()> {
        println!("Server open connection '{:?}'", shake.remote_addr());
        // schedule a timeout to send a ping every 5 seconds
        // self.out.timeout(5_000, PING)?;
        // schedule a timeout to close the connection if there is no activity for 30 seconds
        // self.out.timeout(30_000, EXPIRE)
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> WSResult<()> {
        let json_string = msg.as_text()?;
        let message: ClientMessage = serde_json::from_str(json_string)?;

        // println!("Received {:?}", message);

        // match message.payload
        // {
        // Event::GetChannels => handle_get_channels(self),
        // Event::CreateChannel(channel_name) => handle_create_channel(self, channel_name),
        // Event::SetName(new_name) => handle_set_name(self, new_name),
        // Event::JoinChannel(channel_name) => handle_join_channel(self, channel_name),
        // Event::SendMessage(message) => handle_send_message(self, message),
        // _ => Ok(())
        // }

        Ok(self.out.send(Message::Text(format!("echo {}", "gg")))?) // simple echo
    }

    fn on_error(&mut self, err: ws::Error) {
        // Shutdown on any error
        println!("Shutting down server for error: {}", err);
        self.out.shutdown().unwrap();
    }
}
