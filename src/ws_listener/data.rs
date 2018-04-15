use ws::{listen, CloseCode, OpCode, Sender, Response, Frame, Handler, Request, Handshake, Message, Result, Error, ErrorKind};

// This handler sends some data to the client and then terminates the connection on the first
// message received, presumably confirming receipt of the data
pub struct Data {
    pub ws: Sender,
    pub data: Vec<&'static str>,
}

impl Handler for Data {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        for msg in self.data.iter() {
            try!(self.ws.send(*msg))
        }
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Data handler received a message: {}", msg);
        println!("Data handler going down.");
        self.ws.close(CloseCode::Normal)
    }
}