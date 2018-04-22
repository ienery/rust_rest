use rocksdb::{DB, Writable, Direction, IteratorMode, Options};
use std::str;

use serde_json;
use serde_json::{Value, Map, Error};
use serde_json::Value::Object;
use serde;

use super::record::{Record};
use super::config::{get_db_transact_path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transact {
    pub record: Record,
    pub transact_id: String,
    pub parent_transact_id: String,
    pub timestamp: i64
}

// Получить все транзвкции.
pub fn get_transacts() -> Option<Vec<Transact>> {
    let db_transact_path = get_db_transact_path();
    let db = DB::open_default(&db_transact_path).unwrap();

    let mut transacts: Vec<Transact> = Vec::new();

    let iter = db.iterator(IteratorMode::Start);
    for (_, value) in iter {
        let value_str = str::from_utf8(&value).unwrap();
        let transact: Transact = serde_json::from_str(value_str).unwrap();
        transacts.push(transact);
    }

    Some(transacts)
}

/// Удалить список транзакций.
pub fn delete_list_transacts(transacts: &Vec<Transact>) {
    let db_transact_path = get_db_transact_path();
    let db = DB::open_default(&db_transact_path).unwrap();

    // Удалить сохраненные транзакции.
    for transact in transacts.iter() {
        let transact_id = &transact.transact_id;
        db.delete(&transact_id.as_bytes());
    }
}