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

// FIXME дублирование
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    transacts: Vec<Transact>,
    block_id: String,
    parent_block_id: String,
    timestamp: i64,
    block_no: i64
}

// Транзакции во всех блоках.
pub fn read_blocks_transacts(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read blocks transacts ===");

    let mut db = DB::open_default("./rocksdb/block").unwrap();
    let mut iter = db.iterator(IteratorMode::Start);
    let mut transact_all: Vec<Transact> = Vec::new();
    for (key, value) in iter {
        let k = str::from_utf8(&key).unwrap();
        let v = str::from_utf8(&value).unwrap();
        let block: Block = serde_json::from_str(&v).unwrap();

        // Транзакции в блоке.
        let transacts = block.transacts;
        for transact in transacts {
            println!("transact {:?}", transact);
            transact_all.push(transact);
        }

        //blocks.push(block);
    }

    // for block in &blocks {
    //     println!("Saw {:?}", block);
    // };

    let result = json!({
        "success": true,
        "body": {
            "transact_all": transact_all
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}

#[derive(Debug, Clone, Deserialize)]
struct RequestTransact {
    transact_id: String
}

// Одна транзакции во всех блоках.
pub fn read_blocks_transact_one(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read blocks transact one ===");
    let struct_body = req.get::<bodyparser::Struct<RequestTransact>>();

    if let Ok(Some(request_read_transact)) = struct_body {
        let transact_id = request_read_transact.transact_id;
        println!("transact_id {:?}", transact_id);

        let mut db = DB::open_default("./rocksdb/block").unwrap();
        let mut iter = db.iterator(IteratorMode::Start);
        for (key, value) in iter {
            let k = str::from_utf8(&key).unwrap();
            let v = str::from_utf8(&value).unwrap();
            let block: Block = serde_json::from_str(&v).unwrap();
            // Транзакции в блоке.
            let transacts = block.transacts;
            for transact in transacts {
                //transact_all.push(transact);

                if (transact_id == transact.transact_id) {
                    let result = json!({
                        "success": true,
                        "body": {
                            "transact": transact
                        }
                    });

                    return Ok(Response::with((content_type, status::Ok, result.to_string())))
                } 

            }

            //blocks.push(block);
        }
    }
    
    
    
    // for block in &blocks {
    //     println!("Saw {:?}", block);
    // };

    // let result = json!({
    //     "success": true,
    //     "body": {
    //         "transact_all": transact_all
    //     }
    // });
    Ok(Response::with((content_type, status::Ok, "{}")))
    //Ok(Response::with((content_type, status::Ok, result.to_string())))
}
