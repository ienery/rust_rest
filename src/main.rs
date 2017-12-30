extern crate iron;
extern crate router;
extern crate rocksdb;
extern crate params;

extern crate bodyparser;
extern crate persistent;
extern crate serde;

extern crate rand;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

use iron::{Iron};

use persistent::Read;
use iron::status;
use iron::prelude::*;

use router::{Router};

mod handlers;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    //let mut db = DB::open_default("./storage").unwrap();

    let mut router = Router::new();
    router.get("/", handlers::handler, "handler");
    router.get("/all", handlers::query_handler_all, "query_handler_all");
    // router.get("/:query", handlers::query_handler, "query_handler");

    router.post("/createrecord", handlers::create_record, "create_record");
    router.post("/readrecord", handlers::read_record, "read_record");
    router.post("/deleterecord", handlers::delete_record, "delete_record");
    router.post("/readrecords", handlers::read_records, "read_records");

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();

    // fn handler(_: &mut Request) -> IronResult<Response> {
    //     Ok(Response::with((status::Ok, "OK")))
    // }

    // fn query_handler(req: &mut Request) -> IronResult<Response> {
    //     let mut db = DB::open_default("./storage").unwrap();
    //     db.put(b"my newkey2", b"my newValue2");

    //     let ref query = req.extensions.get::<Router>()
    //         .unwrap().find("query").unwrap_or("/");
    //     Ok(Response::with((status::Ok, *query)))
    // }

    // fn query_handler2(req: &mut Request) -> IronResult<Response> {
    //     let mut db = DB::open_default("./storage").unwrap();
    //     match db.get(b"my newkey2") {
    //         Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
    //         Ok(None) => println!("value not found"),
    //         Err(e) => println!("operational problem encountered: {}", e),
    //     }

    //     let content_type = "application/json".parse::<Mime>().unwrap();
    //     let req_ref = req.get_ref::<Params>();

    //     println!("{:?}", req_ref);
    //     Ok(Response::with((content_type, status::Ok, "{\"success\": \"true\"}")))
    // }
}
