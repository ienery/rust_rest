extern crate iron;
extern crate hyper_native_tls;
extern crate router;
extern crate rocksdb;
extern crate params;

extern crate bodyparser;
extern crate persistent;
extern crate serde;

extern crate rand;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate mount;
extern crate staticfile;

extern crate open;

use iron::{Iron};

use persistent::Read;
use iron::status;
use iron::prelude::*;
use hyper_native_tls::NativeTlsServer;

use mount::Mount;
use router::{Router};
use staticfile::Static;

use std::path::Path;

mod handlers_transact;
mod handlers_block;
mod handlers_point;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    open::that("https://localhost:3000");
    println!("Rust REST starting ...");

    //let mut db = DB::open_default("./storage").unwrap();

    let mut router = Router::new();
    router.get("/", handlers_transact::handler, "handler");
    router.get("/all", handlers_transact::query_handler_all, "query_handler_all");
    // router.get("/:query", handlers::query_handler, "query_handler");

    router.post("/transact/create", handlers_transact::create_transact, "create_transact");
    router.post("/transact/read", handlers_transact::read_transact, "read_transact");
    router.post("/transact/delete", handlers_transact::delete_transact, "delete_transact");
    router.post("/transacts/read", handlers_transact::read_transacts, "read_transacts");

    router.post("/block/create", handlers_block::create_block, "create_block");
    router.post("/block/read", handlers_block::read_block, "read_block");
    router.post("/blocks/read", handlers_block::read_blocks, "read_blocks");
    router.post("/blockstransacts/read", handlers_block::read_blocks_transacts, "read_blocks_transacts");
    router.post("/blockstransact/read", handlers_block::read_blocks_transact_one, "read_blocks_transact_one");
    router.post("/blocktransact/read", handlers_block::read_block_transact, "read_block_transact");

    router.post("/point/read", handlers_point::read_point, "read_point");
    router.post("/points/read", handlers_point::read_points, "read_points");

    let mut mount = Mount::new();
    mount
        .mount("/", Static::new(Path::new("ui/out/index.html")))
        .mount("/assets", Static::new(Path::new("ui/out/assets")))
        .mount("/transact", Static::new(Path::new("ui/out/index.html")))
        .mount("/transact-create", Static::new(Path::new("ui/out/index.html")))
        .mount("/transact-details", Static::new(Path::new("ui/out/index.html")))
        .mount("/transacts", Static::new(Path::new("ui/out/index.html")))
        .mount("/rest", router);
        
    let mut chain = Chain::new(mount);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    let ssl = NativeTlsServer::new("ssl/identity.p12", "mypass").unwrap();
    Iron::new(chain).http("localhost:3000", /*ssl*/).unwrap();

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
