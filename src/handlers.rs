use iron::prelude::*;
use iron::{Request, Response, IronResult};
use iron::status;

use params::{Params, Value};

use iron::mime::Mime;
use router::{Router};

use rocksdb::{DB, Writable};

pub fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "OK")))
    }

pub fn query_handler(req: &mut Request) -> IronResult<Response> {
    let mut db = DB::open_default("./storage").unwrap();
    db.put(b"my newkey", b"my newValue");

    let ref query = req.extensions.get::<Router>()
        .unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

pub fn query_handler2(req: &mut Request) -> IronResult<Response> {
    let mut db = DB::open_default("./storage").unwrap();
    match db.get(b"my newkey") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    let content_type = "application/json".parse::<Mime>().unwrap();
    let map = req.get_ref::<Params>().unwrap();
    println!("{:?}", map);

    // match map.find(&["key"]) {
    //     Some(&Value::String(ref key)) => assert_eq!(key, "value"),
    //     _ => panic!("Unexpected parameter type!"),
    // }

    let mut value = "";
    if let Some(&Value::String(ref key)) = map.find(&["key"]) {
        //assert_eq!(key, "value");
        println!("{}", key);
        value = key;
        println!("{}", value);
    } else {
        panic!("Unexpected parameter type!");
    }
   
    let result = json!({
        "success": true,
        "body": {
            "key": value
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}