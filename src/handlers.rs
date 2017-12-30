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
   
    let mut db = DB::open_default("./rocksdb/data").unwrap();
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
struct RecordData {
    userId: String,
	periodYear: String,
	periodMonth: String,
	readings: String,
    sendDateTime: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Record {
    data: RecordData,
    record_id: String
}

pub fn create_record(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== create block ===");
    let json_body = req.get::<bodyparser::Json>();

    if let Ok(Some(json_parse)) = json_body {
        if let Object(payload) = json_parse {
            println!("payload {:?}", payload);
            let mut record = &payload["record"];
            println!("record {:?}", record);
            if let &Object(ref record_deser) = record {
                println!("record_deser {:?}", record_deser);

                let record_id = rand::random::<(u64)>().to_string();
                println!("record_id {:?}", record_id);

                let record_data = RecordData {
                    userId: record_deser["userId"].as_str().unwrap().to_string(),
                    periodYear: record_deser["periodYear"].as_str().unwrap().to_string(),
                    periodMonth: record_deser["periodMonth"].as_str().unwrap().to_string(),
                    readings: record_deser["readings"].as_str().unwrap().to_string(),
                    sendDateTime: record_deser["sendDateTime"].as_str().unwrap().to_string()
                };

                println!("record_data {:?}", record_data);

                let record = Record {
                    data: record_data,
                    record_id: record_id.to_owned()
                };
                println!("record {:?}", record);

                let record_json = serde_json::to_string(&record).unwrap();
                println!("record_json {:?}", record_json);

                let mut db = DB::open_default("./rocksdb/data").unwrap();
                db.put(&record_id.as_bytes(), &record_json.as_bytes()).unwrap();

                let result = json!({
                    "success": true,
                    "body": {
                        "recordId": record_id,
                        "data": record.data
                    }
                });

                return Ok(Response::with((content_type, status::Ok, result.to_string())))
            }

            
        }
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}

#[derive(Debug, Clone, Deserialize)]
struct RequestReadRecord {
    recordId: String
}

pub fn read_record(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read block ===");
    let struct_body = req.get::<bodyparser::Struct<RequestReadRecord>>();
    println!("struct_body {:?}", struct_body);

    if let Ok(Some(request_read_record)) = struct_body {
        println!("request_read_record {:?}", request_read_record);
        let record_id = request_read_record.recordId;

        let mut db = DB::open_default("./rocksdb/data").unwrap();
        if let Ok(Some(record_data)) = db.get(&record_id.as_bytes()) {
            let record_str = record_data.to_utf8().unwrap();
            println!("record_str {:?}", record_str);
            let record: Record = serde_json::from_str(&record_str).unwrap();
            println!("record {:?}", record);

            let result = json!({
                "success": true,
                "body": {
                    "record": record
                }
            });

            return Ok(Response::with((content_type, status::Ok, result.to_string())))
        }
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}

pub fn delete_record(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read block ===");
    let struct_body = req.get::<bodyparser::Struct<RequestReadRecord>>();
    println!("struct_body {:?}", struct_body);

    if let Ok(Some(request_read_record)) = struct_body {
        println!("request_delete_record {:?}", request_read_record);
        let record_id = request_read_record.recordId;

        let mut db = DB::open_default("./rocksdb/data").unwrap();
        db.delete(&record_id.as_bytes());

        let result = json!({
            "success": true,
            "body": {
                "recordId": record_id
            }
        });

        return Ok(Response::with((content_type, status::Ok, result.to_string())))
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}

pub fn read_records(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read records ===");

    let mut db = DB::open_default("./rocksdb/data").unwrap();
    let mut iter = db.iterator(IteratorMode::Start);
    let mut records: Vec<Record> = Vec::new();
    for (key, value) in iter {
        let k = str::from_utf8(&key).unwrap();
        let v = str::from_utf8(&value).unwrap();
        println!("Saw {:?} {:?}", k, v);
        let record: Record = serde_json::from_str(&v).unwrap();
        records.push(record);
    }
    println!("records {:?}", records);

    let result = json!({
        "success": true,
        "body": {
            "records": records
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}