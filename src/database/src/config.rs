pub fn get_db_root () -> String {
    let db_root = String::from("./rocksdb");
    db_root
}

pub fn get_db_block () -> String {
    let db_block = String::from("/block");
    let db_root = get_db_root();
    let db_root_block = format!("{}{}", db_root, db_block);
    db_root_block
}
