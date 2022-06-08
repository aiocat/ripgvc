// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// IMPORTANT:
// This part is edited version of the <https://github.com/ndelvalle/rustapi/blob/cd06b205551232eaa7b95ab5801db9a5785cab4e/src/database.rs>

use mongodb::error::Error as MongoError;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

// The Rust compiler is allowed to assume that the value a shared reference
// points to will not change while that reference lives. CONNECTION is unsafely
// mutated only once on the setup function (This function is called only once).
static mut CONNECTION: Option<Database> = None;
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

// user struct
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub views: usize,
}

// setup mongo server
pub async fn setup() -> Result<(), MongoError> {
    let exchange =
        IS_INITIALIZED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed);
    let can_setup = exchange == Ok(false);

    if !can_setup {
        panic!("Database already initialized");
    }

    let adress = std::env::var("MONGO_URL")
        .ok()
        .and_then(|s| Some(s))
        .unwrap_or("mongodb://localhost:27017/".to_string());

    let connection = mongodb::Client::with_uri_str(adress)
        .await?
        .database("ripgvc");

    unsafe {
        CONNECTION = Some(connection);
    };

    Ok(())
}

// get database connection from global variable, uses unsafe code.
pub fn get_connection() -> &'static Database {
    unsafe {
        CONNECTION
            .as_ref()
            .expect("Database connection not initialized")
    }
}
