use ws::{listen, CloseCode, OpCode, Sender, Response, Frame, Handler, Request, Handshake, Message, Result, Error, ErrorKind};

// This handler simply echoes all messages back to the client
pub struct Echo {
    pub ws: Sender,
}

impl Handler for Echo {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Echo handler received a message: {}", msg);
        let data = vec!["Echoone", "Echotwo", "Echothree", "Echofour", "Echofive"];
        for msg in data.iter() {
            try!(self.ws.send(*msg));
        }
        self.ws.close_with_reason(CloseCode::Error, "Not Implemented.")
        //self.ws.send(msg)
    }

}