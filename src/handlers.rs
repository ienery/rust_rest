use iron::prelude::*;
use iron::{Request, Response, IronResult};
use iron::status;

use iron::mime::Mime;
use router::{Router};

use rocksdb::{DB, Writable};
use bodyparser;
use serde_json;
use serde_json::Value;
use serde_json::Value::Object;
use serde;

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

pub fn query_handler3(req: &mut Request) -> IronResult<Response> {
    let mut db = DB::open_default("./storage").unwrap();
    match db.get(b"my newkey") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    let content_type = "application/json".parse::<Mime>().unwrap();

    // match map.find(&["key"]) {
    //     Some(&Value::String(ref key)) => assert_eq!(key, "value"),
    //     _ => panic!("Unexpected parameter type!"),
    // }
   
    let result = json!({
        "success": true,
        "body": {
            "key": "22"
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}

#[derive(Debug, Clone, Deserialize)]
struct MyStructure {
    key: String,
    value: String
}

pub fn query_handler2(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    //let json_body = req.get::<bodyparser::Json>();
    //println!("{:?}", json_body);

    //let mut db = DB::open_default("./storage").unwrap();

    if let Ok(Some(struct_body)) = req.get::<bodyparser::Struct<MyStructure>>() {
        println!("{:?}", struct_body);
        println!("{} {}", struct_body.key, struct_body.value);

        let mut k = struct_body.key;
        let mut v = struct_body.value;

        let mut db = DB::open_default("./storage").unwrap();
        db.put(&k.as_bytes(), &v.as_bytes());

        if let Ok(getVal) = db.get(&k.as_bytes()) {
            if let Some(value) = getVal {
                //println!("retrieved value {}", value.to_utf8().unwrap());
                let val = value.to_utf8().unwrap();
                let key = k;

                let result = json!({
                    "success": true,
                    "body": {
                        key: val
                    }
                });
                return Ok(Response::with((content_type, status::Ok, result.to_string())))
            }
        }
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}
// pub fn update_record(req: &mut Request) -> IronResult<Response> {
//     let content_type =  "application/json".parse::<Mime>().unwrap();
//     let map = req.get_ref::<Params>().unwrap();
 
//     let mut value = "";
//     if let Some(&Value::String(ref key)) = map.find(&["key"]) {
        
//     }

// }