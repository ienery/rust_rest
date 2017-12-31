use iron::prelude::*;
use iron::{Request, Response, IronResult};
use iron::status;

use iron::mime::Mime;
use router::{Router};

use rocksdb::{DB, Writable, Direction, IteratorMode};
use bodyparser;
use serde_json;
use serde_json::{Value, Map, Error};
use serde_json::Value::Object;
use serde;

use std::str;
use rand;

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
// #[derive(Debug, Clone, Deserialize)]
// struct NestedStructure {
//     a: String
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Payload {
    key1: String,
    key2: String
}

pub fn query_handler2(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    let json_body = req.get::<bodyparser::Json>();
    
    //let struct_body = req.get::<bodyparser::Struct<MyStructure>>();
    //let body = req.get::<bodyparser::Raw>();
    //println!("{:?}", json_body);
    println!("====================");
    if let Ok(Some(json_parse)) = json_body {
        //println!("json_parse: {:?}", json_parse);
        if let Object(object) = json_parse {
            //println!("{:?}", object);
            let field_names: Vec<_> = object.keys().collect();
            //println!("{:?}", field_names);
            let key = field_names[0];
            //let value = object[key].as_str().unwrap();

            let value_obj = &object[key];
            println!("value_obj: {:?}", value_obj);

            let value_str = value_obj.to_string();
            //println!("value_str: {:?}", value_str);

            let mut db = DB::open_default("./storage").unwrap();
            db.put(&key.as_bytes(), &value_str.as_bytes()).unwrap();

            if let Ok(get_val) = db.get(&key.as_bytes()) {
                if let Some(val) = get_val {
                    let val_result = val.to_utf8().unwrap();
                    let value_parse: Payload = serde_json::from_str(&val_result).unwrap();
                    println!("value_parse db: {:?}", value_parse);

                    let key_result = key.to_string();
                    
                    let result = json!({
                        "success": true,
                        "body": {
                            key_result: {
                                "key1": value_parse.key1,
                                "key2": value_parse.key2
                            }
                        }
                    });

                    return Ok(Response::with((content_type, status::Ok, result.to_string())))
                } 
            }
        }
        /*
        let value = one[key].as_str().unwrap();
        //let str1 = one.to_string();
        //let v: Value = serde_json::from_str(&str1).unwrap();
        //if let Some(z) = one["key"].as_str() {
            println!("{}", value);
        //}   
        // println!("{:?}", one);
        // if let Object(two) = one {
        //     println!("{:?}", two["key"]);
        //     if let Some(val3) = two["key"].as_str() {
        //         println!("{:?}", val3);
        //     }
        // }
        */
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}

pub fn query_handler_all(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
   
    let mut db = DB::open_default("./rocksdb/records").unwrap();
    let mut iter = db.iterator(IteratorMode::Start);
    for (key, value) in iter {
        let k = str::from_utf8(&key).unwrap();
        let v = str::from_utf8(&value).unwrap();
        println!("Saw {:?} {:?}", k, v); 
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
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Record {
    user_id: String,
	period_year: String,
	period_month: String,
	readings: String,
    send_date_time: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transact {
    record: Record,
    transact_id: String
}

pub fn create_transact(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== create transact ===");
    let json_body = req.get::<bodyparser::Json>();

    if let Ok(Some(json_parse)) = json_body {
        if let Object(payload) = json_parse {
            //println!("payload {:?}", payload);
            let mut transact = &payload["record"];
            //println!("record {:?}", record);
            if let &Object(ref record_transact) = transact {
                //println!("record_deser {:?}", record_deser);

                let transact_id = rand::random::<(u64)>().to_string();
                //println!("record_id {:?}", record_id);

                let record = Record {
                    user_id: record_transact["user_id"].as_str().unwrap().to_string(),
                    period_year: record_transact["period_year"].as_str().unwrap().to_string(),
                    period_month: record_transact["period_month"].as_str().unwrap().to_string(),
                    readings: record_transact["readings"].as_str().unwrap().to_string(),
                    send_date_time: record_transact["send_date_time"].as_str().unwrap().to_string()
                };

                //println!("record_data {:?}", record_data);

                let transact = Transact {
                    record: record,
                    transact_id: transact_id.to_owned()
                };
                //println!("record {:?}", record);

                let transact_json = serde_json::to_string(&transact).unwrap();
                //println!("record_json {:?}", record_json);

                let mut db = DB::open_default("./rocksdb/records").unwrap();
                db.put(&transact_id.as_bytes(), &transact_json.as_bytes()).unwrap();

                let result = json!({
                    "success": true,
                    "body": {
                        "transactId": transact_id,
                        "record": transact.record
                    }
                });

                return Ok(Response::with((content_type, status::Ok, result.to_string())))
            }

            
        }
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}

#[derive(Debug, Clone, Deserialize)]
struct RequestTransact {
    transact_id: String
}

pub fn read_transact(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read transact ===");
    let struct_body = req.get::<bodyparser::Struct<RequestTransact>>();

    if let Ok(Some(request_read_transact)) = struct_body {
        let transact_id = request_read_transact.transact_id;

        let mut db = DB::open_default("./rocksdb/records").unwrap();
        if let Ok(Some(transact_data)) = db.get(&transact_id.as_bytes()) {
            let transact_str = transact_data.to_utf8().unwrap();
            let transact: Transact = serde_json::from_str(&transact_str).unwrap();

            let result = json!({
                "success": true,
                "body": {
                    "transact": transact
                }
            });

            return Ok(Response::with((content_type, status::Ok, result.to_string())))
        }
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}

pub fn delete_transact(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read transacts ===");
    let struct_body = req.get::<bodyparser::Struct<RequestTransact>>();

    if let Ok(Some(request_read_transact)) = struct_body {
        let transact_id = request_read_transact.transact_id;

        let mut db = DB::open_default("./rocksdb/records").unwrap();
        db.delete(&transact_id.as_bytes());

        let result = json!({
            "success": true,
            "body": {
                "transact_id": transact_id
            }
        });

        return Ok(Response::with((content_type, status::Ok, result.to_string())))
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}

pub fn read_transacts(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read transacts ===");

    let mut db = DB::open_default("./rocksdb/records").unwrap();
    let mut iter = db.iterator(IteratorMode::Start);
    let mut transacts: Vec<Transact> = Vec::new();
    for (key, value) in iter {
        let k = str::from_utf8(&key).unwrap();
        let v = str::from_utf8(&value).unwrap();
        let transact: Transact = serde_json::from_str(&v).unwrap();
        transacts.push(transact);
    }

    let result = json!({
        "success": true,
        "body": {
            "transacts": transacts
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}