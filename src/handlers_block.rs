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
    send_date_time: String
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

// Поиск последнео блока по отсутствию хеша
// Блоки без метки времени.
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

    //println!("block {:?}", block);
    let mut db_block = DB::open_default("./rocksdb/block").unwrap();

    // Найти предыдущий блок.
    //let mut db_block = DB::open_default("./rocksdb/block").unwrap();
    let mut iter = db_block.iterator(IteratorMode::Start);
    let mut blocks: Vec<Block> = Vec::new();
    for (key, value) in iter {
        let k = str::from_utf8(&key).unwrap();
        let v = str::from_utf8(&value).unwrap();
        let block: Block = serde_json::from_str(&v).unwrap();
        blocks.push(block);
        //println!("Saw {:?}", block);
    }

    //println!("blocks.len {:?}", blocks.len());

    // Если блоков не было, то это первый блок.
    let mut parent_block_id = "genesis";
    let mut block_no = 1;

    if (blocks.len() == 0) {
        //println!("blocks.len 0");
    } else {
        // Если блоки были, то найти последний в цепочке,
        // Он ни для кого не является родителем.
        parent_block_id = "no genesis";

        for block_out in &blocks {
            let block_out_id = &block_out.block_id;
            let block_out_no = &block_out.block_no;
            //println!("Saw block_out_id {:?}", block_out_id);
            //let mut hasParent = false;
            let mut hasParents: Vec<bool> = Vec::new();
            for block_in in &blocks {
                let block_in_parent_block_id = &block_in.parent_block_id;
                //println!("Saw block_in_parent_block_id {:?}", block_in_parent_block_id);

                if (block_out_id == block_in_parent_block_id) {
                    hasParents.push(true);
                } else {
                    hasParents.push(false);
                    //parent_block_id = block_out_id;
                }
            }
      
            let mut hasParent = false;
            //let hasParentsLen = hasParents.len();

            for (index, hasParentOne) in hasParents.into_iter().enumerate() {
                //println!("index hasParentOne {}: {}", index, hasParentOne);
                //println!("hasParents.len() {}", hasParentsLen);
                
                if (hasParentOne == true) {
                    hasParent = true;
                    break;
                } 

                hasParent = false;
                
            }

            //println!("hasParent {}", hasParent);
            if (hasParent == false) {
                parent_block_id = block_out_id;
                block_no = block_out_no + 1;
            }
            
        };
    }

    //println!("parent_block_id {}", parent_block_id);

    let dateTimeUtc: DateTime<Utc> = Utc::now();
    let timestamp = dateTimeUtc.timestamp();
    //println!("timestamp {:?}", timestamp);

    let block = Block {
        transacts: transacts,
        block_id: block_id.to_owned(),
        parent_block_id: parent_block_id.to_owned(),
        timestamp: timestamp,
        block_no: block_no
    };

    
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

    // for block in &blocks {
    //     println!("Saw {:?}", block);
    // };

    let result = json!({
        "success": true,
        "body": {
            "blocks": blocks
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}

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