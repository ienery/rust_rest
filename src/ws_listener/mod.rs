use std::str::from_utf8;
use time;
use ws::{listen, CloseCode, OpCode, Sender, Response, Frame, Handler, Request, Handshake, Message, Result, Error, ErrorKind};
use ws::util::{Token, Timeout};

pub mod pong;
pub mod echo;
pub mod data;

// A WebSocket handler that routes connections to different boxed handlers by resource
struct Router {
    sender: Sender,
    inner: Box<Handler>,
}

impl Handler for Router {
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {

        // Clone the sender so that we can move it into the child handler
        let out = self.sender.clone();

        match req.resource() {
            "/echo" => self.inner = Box::new(echo::Echo { 
                ws: out 
            } ),

            "/pong" => self.inner = Box::new(pong::Pong { 
                ws: out,
                ping_timeout: None,
                expire_timeout: None,
            } ),

            // Route to a data handler
            "/data/one" => self.inner = Box::new(data::Data {
                ws: out,
                data: vec!["one", "two", "three", "four", "five"]
            }),

            // Route to another data handler
            "/data/two" => self.inner = Box::new(data::Data {
                ws: out,
                data: vec!["いち", "二", "さん", "四", "ご"]
            }),

            // Use a closure as the child handler
            "/closure" => self.inner = Box::new(move |msg: Message| {
                println!("Got a message on a closure handler: {}", msg);

                let data = vec!["one1", "two2", "three4", "four5", "five6"];
                for msg in data.iter() {
                    try!(out.send(*msg));
                }
                out.close_with_reason(CloseCode::Error, "Not Implemented.")
            }),

            // Use the default child handler, NotFound
            _ => (),
        }

        // Delegate to the child handler
        self.inner.on_request(req)
    }

    // Pass through any other methods that should be delegated to the child.
    //
    // You could probably use a macro for this if you have many different
    // routers or were building some sort of routing framework.

    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.inner.on_message(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: Error) {
        self.inner.on_error(err);
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        self.inner.on_timeout(event)
    }

    fn on_new_timeout(&mut self, event: Token, timeout: Timeout) -> Result<()> {
        self.inner.on_new_timeout(event, timeout)
    }

    fn on_frame(&mut self, frame: Frame) -> Result<Option<Frame>> {
        self.inner.on_frame(frame)
    }
}

// This handler returns a 404 response to all handshake requests
struct NotFound;

impl Handler for NotFound {

    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        // This handler responds to all requests with a 404
        let mut res = try!(Response::from_request(req));
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }

}





pub fn handler_ws () {
    // Listen on an address and call the closure for each connection
    if let Err(error) = listen("localhost:3001", |out| {

            // Use our router as the handler to route the new connection
            Router {
                sender: out,
                // Default to returning a 404 when the route doesn't match.
                // You could default to any handler here.
                inner: Box::new(NotFound),
            }

        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to {:?}", error);
        }
}