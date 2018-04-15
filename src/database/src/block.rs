use rocksdb::{DB, Writable, Direction, IteratorMode, Options};
use std::str;

use serde_json;
use serde_json::{Value, Map, Error};
use serde_json::Value::Object;
use serde;

use super::transact::{Transact};
use super::config::{get_db_block};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    transacts: Vec<Transact>,
    block_id: String,
    parent_block_id: String,
    timestamp: i64,
    block_no: i64
}

/// Получить все блоки. 
pub fn get_blocks () -> Vec<Block> {
    let db_block_path = get_db_block();
    let db = DB::open_default(&db_block_path).unwrap();

    let iter = db.iterator(IteratorMode::Start);
    let mut blocks: Vec<Block> = Vec::new(); 
    for (_, value) in iter { 
        let v = str::from_utf8(&value).unwrap();
        let block: Block = serde_json::from_str(&v).unwrap();
        blocks.push(block);
    }

    blocks
}