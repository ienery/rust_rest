use super::record::{Record};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transact {
    record: Record,
    transact_id: String,
    parent_transact_id: String,
    timestamp: i64
}