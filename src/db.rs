use mongodb::bson::doc;
use mongodb::{Client, Collection, Database};
use rocket::futures::TryStreamExt;
use rocket::response::Debug;
use std::env::var;

use crate::types::{Capacity, Host, Performance, Result, Uptime};

// AppDbPool is managed by Rocket as a State, which means it is available across threads.
// Type mongodb::Database (starting v2.0.0 of mongodb driver) represents a connection pool to db,
// therefore it is thread-safe and can be passed around between threads via State
pub struct AppDbPool {
    pub db: Database,
}

// Initialize database and return in form of an AppDbPool
pub async fn init_db_pool() -> AppDbPool {
    let mongo_uri: String = var("MONGO_URI").expect("MONGO_URI must be set in the env");
    let client = Client::with_uri_str(mongo_uri).await.unwrap();

    AppDbPool {
        db: client.database("pjs-test"),
    }
}

// Ping database and return a string if successful
// Timeouts to 500 error response
pub async fn ping_database(db: &Database) -> Result<String> {
    db.run_command(doc! {"ping": 1}, None).await?;
    Ok(format!("Connected to db."))
}

// Find a value of uptime for host identified by its name in a collection `performance_summary`
// Returns 404 if not found
pub async fn host_uptime(name: String, db: &Database) -> Option<Uptime> {
    let records: Collection<Performance> = db.collection("performance_summary");

    if let Some(host) = records.find_one(Some(doc! {"name": name}), None).await.unwrap() {
        return Some(Uptime {
            uptime: host.uptime,
        });
    }
    None
}

// Calculate network capacity from all the records in `performance_summary` collection
pub async fn network_capacity(db: &Database) -> Result<Capacity> {
    let records: Collection<Performance> = db.collection("performance_summary");
    let cursor = records.find(None, None).await?;

    // cursor is a stream so it requires fold() from StreamExt
    cursor.try_fold(
        Capacity {
            total_hosts: 0,
            read_only: 0,
            source_chain: 0,
        },
        |mut acc, el| async move {
            acc.calc_capacity(el.uptime);
            Ok(acc)
        },
    ).await.map_err(Debug)
}

pub async fn list_all_hosts(db: &Database) -> Result<Vec<Host>> {
    let records: Collection<Host> = db.collection("performance_summary");

    let latest_timestamp: u64 = 1631612888888;

    let mut result: Vec<Host> = Vec::new();

    // if let Some(host) = records.find_one(Some(doc! {"name": name}), None).await.unwrap() {
    //     latest_timestamp = host.timestamp;
    // } else {
    //     return Ok(Vec::new())
    // }

    let cursor = records.find(Some(doc! {"timestamp": latest_timestamp}), None).await?;

    cursor.try_collect().await.map_err(Debug)
}
