/// Путь к БД.
pub fn get_db_root_path() -> String {
    let db_root = String::from("./rocksdb");
    db_root
}

/// Путь к блокам.
pub fn get_db_block_path() -> String {
    let db_block = String::from("/block");
    let db_root = get_db_root_path();
    let db_root_block = format!("{}{}", db_root, db_block);
    db_root_block
}

/// Путь к транзакциям.
pub fn get_db_transact_path() -> String {
    let db_transact = String::from("/transact");
    let db_root = get_db_root_path();
    let db_root_transact = format!("{}{}", db_root, db_transact);
    db_root_transact
}