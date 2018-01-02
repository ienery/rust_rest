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

// FIXME дублирование
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    transacts: Vec<Transact>,
    block_id: String
}

pub fn create_block(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== create block ===");

    // Прочитать транзакци из базы.
    let mut db_transact = DB::open_default("./rocksdb/transact").unwrap();
    let mut iter = db_transact.iterator(IteratorMode::Start);
    let mut transacts: Vec<Transact> = Vec::new();
    for (key, value) in iter {
        let k = str::from_utf8(&key).unwrap();
        let v = str::from_utf8(&value).unwrap();
        let transact: Transact = serde_json::from_str(&v).unwrap();
        transacts.push(transact);
    }

    // Записать транзакции в блок.
    let block_id = rand::random::<(u64)>().to_string();

    let block = Block {
        transacts: transacts,
        block_id: block_id.to_owned()
    };

    //println!("block {:?}", block);
    let mut db_block = DB::open_default("./rocksdb/block").unwrap();
    let block_json = serde_json::to_string(&block).unwrap();

    db_block.put(&block_id.as_bytes(), &block_json.as_bytes()).unwrap();


    // Удалить сохраненные транзакции.
    for transact in &block.transacts {
        //println!("transact {:?}", transact);
        let transact_id = &transact.transact_id;
        db_transact.delete(&transact_id.as_bytes());
    }

    let result = json!({
        "success": true,
        "body": {
            "block": block
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}

pub fn read_blocks(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read blocks ===");

    let mut db = DB::open_default("./rocksdb/block").unwrap();
    let mut iter = db.iterator(IteratorMode::Start);
    let mut blocks: Vec<Block> = Vec::new();
    for (key, value) in iter {
        let k = str::from_utf8(&key).unwrap();
        let v = str::from_utf8(&value).unwrap();
        let block: Block = serde_json::from_str(&v).unwrap();
        blocks.push(block);
    }

    let result = json!({
        "success": true,
        "body": {
            "blocks": blocks
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}