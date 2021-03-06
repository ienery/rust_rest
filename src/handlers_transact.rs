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
use chrono::prelude::*;

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
   
    let mut db = DB::open_default("./rocksdb/transact").unwrap();
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
    point_id: String,
	period_year: String,
	period_month: String,
	readings: String,
    send_date_timestamp: String,
    period_timestamp: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transact {
    record: Record,
    transact_id: String,
    parent_transact_id: String,
    timestamp: i64
}

// FIXME дублирование.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    transacts: Vec<Transact>,
    block_id: String,
    parent_block_id: String,
    timestamp: i64,
    block_no: i64
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
                //println!("record_id {:?}", record_id);

                // Точка учета.
                let point_id = record_transact["point_id"].as_str().unwrap().to_string();

                let record = Record {
                    user_id: record_transact["user_id"].as_str().unwrap().to_string(),
                    point_id: point_id.to_owned(),
                    period_year: record_transact["period_year"].as_str().unwrap().to_string(),
                    period_month: record_transact["period_month"].as_str().unwrap().to_string(),
                    readings: record_transact["readings"].as_str().unwrap().to_string(),
                    send_date_timestamp: record_transact["send_date_timestamp"].as_str().unwrap().to_string(),
                    period_timestamp: record_transact["period_timestamp"].as_str().unwrap().to_string(),
                };

                //println!("record_data {:?}", record_data);

                let mut parent_transact_id: String  = "genesis".to_string();
                let mut transacts_point: Vec<Transact> = Vec::new();
                // Найти транзакцию с предыдущей связанной записью (record),
                // запись определять по идентификатору точки учета (record.point_id).

                // Сначала поиск по транзакциям не упакованным в блок.
                let mut db_transact = DB::open_default("./rocksdb/transact").unwrap();
                let mut transact_iter = db_transact.iterator(IteratorMode::Start);
                let mut transacts: Vec<Transact> = Vec::new();
                for (key, value) in transact_iter {
                    let k = str::from_utf8(&key).unwrap();
                    let v = str::from_utf8(&value).unwrap();
                    let transact: Transact = serde_json::from_str(&v).unwrap();
                    transacts.push(transact);
                }

                for transact in transacts {
                    let transact_point_id = transact.record.point_id.to_owned();
                    if (point_id == transact_point_id.to_string()) {
                        //println!("Saw transact {:?}", transact_point_id);
                        transacts_point.push(transact);
                    }
                }
                    
                if (transacts_point.len() > 0) {
                    //println!(">0");
                    // Сортировка по убыванию меток времени.
                    transacts_point.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                    // Упростим - просто берем последнюю транзакцию.
                    let transact_id = &transacts_point[0].transact_id;
                    parent_transact_id = transact_id.to_string();
                    
                    // Найти транзакцию, которая не является предыдущей для других транзакций.
                    //println!("last_transact_point {}", parent_transact_id);
                    //parent_transact_id = parent_transact_id1;
                } else {
                    //println!("=0");
                    // Просмотр транзакций внутри блоков.
                    let mut db_block = DB::open_default("./rocksdb/block").unwrap();

                    let mut iter = db_block.iterator(IteratorMode::Start);
                    let mut blocks: Vec<Block> = Vec::new();
                    for (key, value) in iter {
                        let k = str::from_utf8(&key).unwrap();
                        let v = str::from_utf8(&value).unwrap();
                        let block: Block = serde_json::from_str(&v).unwrap();
                       
                        //println!("Saw block {:?}", block);

                        let mut transacts_point: Vec<Transact> = Vec::new();
                        
                        let mut transacts_point  = 0;
                        // Транзакция содержащие запись (record) с point_id
                        for transact in &block.transacts {
                            let transact_point_id = transact.record.point_id.to_owned();
                            if (point_id == transact_point_id.to_string()) {
                                //println!("Saw transact {:?}", transact_point_id);
                                transacts_point = transacts_point + 1;
                            }
                        }

                        //println!("transacts_point {}", transacts_point);

                        if (transacts_point > 0) {
                            //println!("block.transacts_point.len > 0");
                            blocks.push(block);
                        } else {
                            //println!("block.transacts_point.len = 0");
                            continue;
                        }
                    }

                    // Все блоки с транзакцией по point_id.
                    // Сортировка блоков по убыванию меток времени.
                    blocks.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                    for block_point_transacts in blocks {
                        let mut flag_break = false;
                        //println!("Saw blocks point_id timestamp {:?}", block_point_transacts.timestamp);
                        let mut block_transacts = block_point_transacts.transacts;

                        // Сортировка транзакции в блоке по убыванию меток времени.
                        block_transacts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

                        for transact in block_transacts {
                            //println!("Saw blocks transact timestamp {:?}", transact.timestamp);
                            let transact_point_id = transact.record.point_id.to_owned();
                            // Соответствие записи point_id
                            if (transact_point_id == point_id) {
                                //println!("transact.record {:?}", transact.record);
                                // Упростим задачу - берем первую транзакцию по условию совпадения point_id записи
                                // из отсортированных по времени, по убыванию timestamp.
                                let transact_id = &transact.transact_id;
                                parent_transact_id = transact_id.to_string();
                                flag_break = true;
                                break;
                            }
                        }

                        if (flag_break == true) {
                            break;
                        }
                        //println!("++++++");
                    }
                }

                


                let transact_id = rand::random::<(u64)>().to_string();
                
                let dateTimeUtc: DateTime<Utc> = Utc::now();
                let timestamp = dateTimeUtc.timestamp();
    
                let transact = Transact {
                    record: record,
                    transact_id: transact_id.to_owned(),
                    parent_transact_id: parent_transact_id.to_owned(),
                    timestamp: timestamp
                };
                //println!("record {:?}", record);

                let transact_json = serde_json::to_string(&transact).unwrap();
                //println!("record_json {:?}", record_json);
                db_transact.put(&transact_id.as_bytes(), &transact_json.as_bytes()).unwrap();

                let result = json!({
                    "success": true,
                    "body": {
                        "transact": transact
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

        let mut db = DB::open_default("./rocksdb/transact").unwrap();
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

        let mut db = DB::open_default("./rocksdb/transact").unwrap();
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

    let mut db = DB::open_default("./rocksdb/transact").unwrap();
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