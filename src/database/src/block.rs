use rocksdb::{DB, Writable, Direction, IteratorMode, Options};
use std::str;

use serde_json;
use serde_json::{Value, Map, Error};
use serde_json::Value::Object;
use serde;

use rand;
use chrono::prelude::*;

use super::transact;
use super::transact::{Transact};
use super::config::{get_db_block_path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub transacts: Vec<Transact>,
    pub block_id: String,
    pub parent_block_id: String,
    pub timestamp: i64,
    pub block_no: i64
}

impl Block {
    /// Получить транзакции из блока.
    fn get_transact(self, transact_id: String) -> Option<Transact> {
        let transacts = self.transacts;

        for transact in transacts {
            if transact_id == transact.transact_id {
                return Some(transact);
            }

            return None;
        }

        None
    }
}

/// Получить все блоки. 
pub fn get_blocks() -> Vec<Block> {
    let db_block_path = get_db_block_path();
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

/// Получить один блок по id
pub fn get_block(block_id: String) -> Option<Block> {
    let db_block_path = get_db_block_path();
    let db = DB::open_default(&db_block_path).unwrap();

    if let Ok(Some(block_data)) = db.get(&block_id.as_bytes()) {
        let block_str = block_data.to_utf8().unwrap();
        let block: Block = serde_json::from_str(&block_str).unwrap();

        return Some(block);
    }

    None
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockTransact {
    pub block_id: String,
    pub transact_id: String
}

/// Получить транзакцию внутри блока.
pub fn get_block_transact(block_transact: BlockTransact) -> Option<Transact> {
    
    let block_id = block_transact.block_id;

    if let Some(block) = get_block(block_id) {
        let transact_id = block_transact.transact_id;
        if let Some(transact) = block.get_transact(transact_id) {
            return Some(transact);
        }

        return None;
    }

    None
}


/// Создание блока.
pub fn create_block() -> Option<Block>{
    // Транзакции без блоков, готовые для добавления.
    let mut transacts: Vec<Transact> = Vec::new();
    if let Some(transacts_in_db) = transact::get_transacts() {
        transacts = transacts_in_db;
    }

    // Если блоков не было, то это первый блок.
    let mut parent_block_id = "genesis".to_string();
    let mut block_no = 1;
    // Если блоки были - присвоить атрибуты с учетом последнего блока.
    if let Some(last_block) = get_last_block() {
        parent_block_id = last_block.block_id.to_owned();
        block_no = last_block.block_no + 1;
    }

    let block_id = rand::random::<(u64)>().to_string();

    let dateTimeUtc: DateTime<Utc> = Utc::now();
    let timestamp = dateTimeUtc.timestamp();

    // Новый блок.
    let block = Block {
        transacts: transacts,
        block_id: block_id,
        parent_block_id: parent_block_id,
        timestamp: timestamp,
        block_no: block_no
    };

    let block_json = serde_json::to_string(&block).unwrap();

    let db_block_path = get_db_block_path();
    let db = DB::open_default(&db_block_path).unwrap();
    db.put(&block.block_id.as_bytes(), &block_json.as_bytes()).unwrap();

    transact::delete_list_transacts(&block.transacts);

    Some(block)
}

#[derive(Debug, Clone)]
pub struct BlockIdAndNumber {
    pub parent_block_id: String,
    pub block_no: i64
}

/// Сгенерировать идентификатор и номер блока.
pub fn generage_new_block_id_and_number() -> BlockIdAndNumber {
    let mut parent_block_id = "genesis".to_owned();
    let mut block_no = 1;

    BlockIdAndNumber {
        parent_block_id: parent_block_id.to_owned(),
        block_no: block_no
    }
}

pub fn get_last_block() -> Option<Block> {
    let mut blocks = get_blocks();
    //println!("blocks {:?}", blocks);

    if blocks.len() != 0 {
        // Если блоки есть, то находим последний в цепочке.
        for block_out in &blocks {
            let block_out_id = block_out.block_id.to_owned();
            
            let mut hasParents: Vec<bool> = Vec::new();
            for block_in in &blocks {
                let block_in_parent_block_id = block_in.parent_block_id.to_owned();
                if block_out_id == block_in_parent_block_id {
                    hasParents.push(true);
                } else {
                    hasParents.push(false);
                }
            }

            let mut hasParent = false;

            for (_, hasParentCurrent) in hasParents.into_iter().enumerate() {
                if hasParentCurrent == true {
                    hasParent = true;
                    break;
                } 

                hasParent = false; 
            }

            if hasParent == false {
                let block = block_out.clone();
                return Some(block);
            }
        }
    }
    None
}