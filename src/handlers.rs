use iron::prelude::*;
use iron::{Request, Response, IronResult};
use iron::status;

use iron::mime::Mime;
use router::{Router};

use rocksdb::{DB, Writable};
use bodyparser;
use serde_json::Value;
use serde_json::Value::Object;

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

pub fn query_handler2(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    //let json_body = req.get::<bodyparser::Json>();
    //println!("{:?}", json_body);

    let mut db = DB::open_default("./storage").unwrap();

    if let Ok(Some(json_body)) = req.get::<bodyparser::Json>() {
        println!("Parsed body:\n{:?}", json_body);
        if let Object(object) = json_body {
            let field_names: Vec<_> = object.keys().collect();
            // println!("{:?}", field_names);
            // println!("{}", field_names[0].to_string());
            // println!("{}", object[field_names[0]].to_string());

            let mut k = field_names[0].to_string();
            let mut v = object[field_names[0]].to_string();

            println!("{}", k);
            println!("{}", v);
            

            db.put(&k.as_bytes(), &v.as_bytes());
            //db.put(field_names[0].to_string().as_bytes(), object[field_names[0].as_bytes());

            if let Ok(getVal) = db.get(&k.as_bytes()) {
                if let Some(value) = getVal {
                    //println!("retrieved value {}", value.to_utf8().unwrap());
                    let result = value.to_utf8().unwrap();
                    //println!("retrieved value {}", result);

                    // let result2 = json!({
                    //     "success": true,
                    //     "body": {
                    //         key: result
                    //     }
                    // });
                    return Ok(Response::with((content_type, status::Ok, result.to_string())))
                    //return Ok(Response::with((content_type, status::Ok, "{\"test\": \"test2\"}")))
                }
            }
            
        }
    } else {
        panic!("Unexpected parameter type!");
    }
    let result = json!({
        "success": true,
        "body": {
            "key": "value100"
        }
    });

    // match db.get(&k.as_bytes()) {
    //     Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
    //     Ok(None) => println!("value not found"),
    //     Err(e) => println!("operational problem encountered: {}", e),
    // }

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}
// pub fn update_record(req: &mut Request) -> IronResult<Response> {
//     let content_type =  "application/json".parse::<Mime>().unwrap();
//     let map = req.get_ref::<Params>().unwrap();
 
//     let mut value = "";
//     if let Some(&Value::String(ref key)) = map.find(&["key"]) {
        
//     }

// }