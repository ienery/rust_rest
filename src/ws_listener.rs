use std::str::from_utf8;
use time;
use ws::{listen, CloseCode, OpCode, Sender, Response, Frame, Handler, Request, Handshake, Message, Result, Error, ErrorKind};
use ws::util::{Token, Timeout};

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
            "/echo" => self.inner = Box::new(Echo { ws: out } ),

            "/pong" => self.inner = Box::new(Pong { 
                ws: out,
                ping_timeout: None,
                expire_timeout: None,
            } ),

            // Route to a data handler
            "/data/one" => self.inner = Box::new(Data {
                ws: out,
                data: vec!["one", "two", "three", "four", "five"]
            }),

            // Route to another data handler
            "/data/two" => self.inner = Box::new(Data {
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


// This handler simply echoes all messages back to the client
struct Echo {
    ws: Sender,
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

// This handler sends some data to the client and then terminates the connection on the first
// message received, presumably confirming receipt of the data
struct Data {
    ws: Sender,
    data: Vec<&'static str>,
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

const PING: Token = Token(1);
const EXPIRE: Token = Token(2);

/// An example demonstrating how to send and recieve a custom ping/pong frame.
struct Pong {
    ws: Sender,
    ping_timeout: Option<Timeout>,
    expire_timeout: Option<Timeout>,
}

impl Handler for Pong {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // schedule a timeout to send a ping every 5 seconds
        println!("Pong Opne");
        try!(self.ws.timeout(5_000, PING));
        // schedule a timeout to close the connection if there is no activity for 30 seconds
        self.ws.timeout(30_000, EXPIRE)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Pong handler received a message: {}", msg);
        self.ws.send(msg)
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        match event {
            // PING timeout has occured, send a ping and reschedule
            PING => {
                println!("Pong PING");
                try!(self.ws.ping(time::precise_time_ns().to_string().into()));
                self.ping_timeout.take();
                self.ws.timeout(5_000, PING);
                self.ws.send("333")
            },
            // EXPIRE timeout has occured, this means that the connection is inactive, let's close
            EXPIRE => {
                println!("Pong EXPIRE");
                self.ws.close(CloseCode::Away)
            },
            // No other timeouts are possible
            _ => Err(Error::new(ErrorKind::Internal, "Invalid timeout token encountered!")),
        }
    }

    fn on_new_timeout(&mut self, event: Token, timeout: Timeout) -> Result<()> {
        // Cancel the old timeout and replace.
        if event == EXPIRE {
            if let Some(t) = self.expire_timeout.take() {
                try!(self.ws.cancel(t))
            }
            self.expire_timeout = Some(timeout)
        } else {
            // This ensures there is only one ping timeout at a time
            if let Some(t) = self.ping_timeout.take() {
                try!(self.ws.cancel(t))
            }
            self.ping_timeout = Some(timeout)
        }

        Ok(())
    }

    fn on_frame(&mut self, frame: Frame) -> Result<Option<Frame>> {
        println!("Received on_frame.");
        // If the frame is a pong, print the round-trip time.
        // The pong should contain data from out ping, but it isn't guaranteed to.
        if frame.opcode() == OpCode::Pong {
            if let Ok(pong) = try!(from_utf8(frame.payload())).parse::<u64>() {
                let now = time::precise_time_ns();
                println!("RTT is {:.3}ms.", (now - pong) as f64 / 1_000_000f64);
            } else {
                println!("Received bad pong.");
            }
        }

        // Some activity has occured, so reset the expiration
        try!(self.ws.timeout(30_000, EXPIRE));

        // Run default frame validation
        DefaultHandler.on_frame(frame)
    }

}

// For accessing the default handler implementation
struct DefaultHandler;

impl Handler for DefaultHandler {}

pub fn handler_ws () {
    // listen("localhost:3001", |out| {
    //     move |msg| {
    //         out.send(msg)
    //     }
    // }).unwrap();

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