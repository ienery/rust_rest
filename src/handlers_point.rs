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

// FIXME дублирование.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    user_id: String,
    point_id: String,
	period_year: String,
	period_month: String,
	readings: String,
    send_date_timestamp: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transact {
    record: Record,
    transact_id: String,
    parent_transact_id: String,
    timestamp: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    transacts: Vec<Transact>,
    block_id: String,
    parent_block_id: String,
    timestamp: i64,
    block_no: i64
}

pub fn get_point_transact(point_id: String) -> Vec<Transact> {
    let mut db_block = DB::open_default("./rocksdb/block").unwrap();
    let mut iter = db_block.iterator(IteratorMode::Start);
    let mut point_transacts: Vec<Transact> = Vec::new();

    let mut blocks: Vec<Block> = Vec::new();
    for (key, value) in iter {
        let k = str::from_utf8(&key).unwrap();
        let v = str::from_utf8(&value).unwrap();
        let block: Block = serde_json::from_str(&v).unwrap();
        blocks.push(block);
        

        //println!("block {:?}", block);
    }

    // Все блоки с транзакцией по point_id.
    // Сортировка блоков по убыванию меток времени.
    blocks.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    for block in blocks {
        let mut transacts = block.transacts;
        // Сортировка транзакций в блоке по убыванию меток времени.
        transacts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        for transact in transacts {
            if (point_id == transact.record.point_id) {
                point_transacts.push(transact);
            }
        }
    }

    point_transacts
}

#[derive(Debug, Clone, Deserialize)]
struct RequestPoint {
    point_id: String
}

pub fn read_point(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read point ===");
    let struct_body = req.get::<bodyparser::Struct<RequestPoint>>();

    if let Ok(Some(request_read_point)) = struct_body {
        let point_id = request_read_point.point_id;

        // for transact in point_transacts {
        //     println!("transact {:?}", transact);
        //         println!("+++++");
        // }
        let transacts = get_point_transact(point_id);

        let result = json!({
                "success": true,
                "body": {
                    "transacts": transacts
                }
            });

        return Ok(Response::with((content_type, status::Ok, result.to_string())))
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}

#[derive(Debug, Clone, Deserialize)]
struct RequestPoints {
    points_id: Vec<String>
}

pub fn read_points(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("=== read points ===");
    let struct_body = req.get::<bodyparser::Struct<RequestPoints>>();
    if let Ok(Some(request_read_points)) = struct_body {
        let points_id: Vec<String> = request_read_points.points_id;
        let mut transacts: Vec<Transact> = Vec::new();

        for point_id in points_id {
            //println!("point_id {:?}", point_id);
            // Неоптимально.
            let mut transacts_point = get_point_transact(point_id);
            transacts.append(&mut transacts_point);
        }

        //println!("transacts {:?}", transacts);

        let result = json!({
                "success": true,
                "body": {
                    "transacts": transacts
                }
            });

        return Ok(Response::with((content_type, status::Ok, result.to_string())))
    }

    Ok(Response::with((content_type, status::Ok, "{}")))
}