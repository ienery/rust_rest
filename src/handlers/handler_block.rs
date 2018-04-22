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

use database::block;
use database::block::{
    BlockTransact
};

/// Чтение всех блоков.
pub fn read_blocks(req: &mut Request) -> IronResult<Response>{
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read blocks ===");

    let blocks = block::get_blocks();
    let result = json!({
        "success": true,
        "body": {
            "blocks": blocks
        }
    });

    Ok(Response::with((content_type, status::Ok, result.to_string())))
}

#[derive(Clone, Deserialize)]
struct RequestBlock {
    block_id: String
}

/// Чтение одного блока.
pub fn read_block(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read block ===");

    let struct_body = req.get::<bodyparser::Struct<RequestBlock>>();
    if let Ok(Some(request_read_block)) = struct_body {
        let block_id = request_read_block.block_id;
        let block = block::get_block(block_id);
        let result_success = json!({
            "success": true,
            "body": {
                "block": block
            }  
        });

        return Ok(Response::with((content_type, status::Ok, result_success.to_string())))
    }

    let result_failure = json!({
        "success": false
    });

    Ok(Response::with((content_type, status::Ok, result_failure.to_string())))
}

#[derive(Debug, Clone, Deserialize)]
struct RequestBlockTransact {
    block_id: String,
    transact_id: String
}

/// Чтение транзакции внутри блокa.
pub fn read_block_transact(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();

    let result_failure = json!({
        "success": false
    });
    println!("=== read block transact ===");

    let struct_body = req.get::<bodyparser::Struct<RequestBlockTransact>>();
    if let Ok(Some(request_read_block_transact)) = struct_body {
        let block_transact = BlockTransact {
            block_id: request_read_block_transact.block_id,
            transact_id: request_read_block_transact.transact_id
        };

        if let Some(transact) = block::get_block_transact(block_transact) {
            let result_success = json!({
                "success": true,
                "body": {
                    "transact": transact
                }
            });

            return Ok(Response::with((content_type, status::Ok, result_success.to_string())));
        }
        
        return Ok(Response::with((content_type, status::Ok, result_failure.to_string())));
    }

    Ok(Response::with((content_type, status::Ok, result_failure.to_string())))
}

/// Создание блока.
pub fn create_block(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== create block ===");

    if let Some(block) = block::create_block() {
        let result_success = json!({
            "success": true,
            "body": {
                "block": block
            }
        });

        return Ok(Response::with((content_type, status::Ok, result_success.to_string())));
    }

    let result_failure = json!({
        "success": false
    });

    Ok(Response::with((content_type, status::Ok, result_failure.to_string())))
}