use std::str::from_utf8;
use time;
use ws::{listen, CloseCode, OpCode, Sender, Response, Frame, Handler, Request, Handshake, Message, Result, Error, ErrorKind};
use ws::util::{Token, Timeout};

use database::block::{get_blocks};

const PING: Token = Token(1);
const EXPIRE: Token = Token(2);

// For accessing the default handler implementation
struct DefaultHandler;

impl Handler for DefaultHandler {}

/// An example demonstrating how to send and recieve a custom ping/pong frame.
pub struct Pong {
    pub ws: Sender,
    pub ping_timeout: Option<Timeout>,
    pub expire_timeout: Option<Timeout>,
}

impl Handler for Pong {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // schedule a timeout to send a ping every 5 seconds
        println!("Pong Opne");
        //let blocks = get_blocks();
        //println!("blocks {:?}", blocks);
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
                self.ws.send("333444")
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