// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// IMPORTANT:
// This part is edited version of the <https://github.com/ndelvalle/rustapi/blob/cd06b205551232eaa7b95ab5801db9a5785cab4e/src/database.rs>

use mongodb::Database;
use serde::{Deserialize, Serialize};

// user struct
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub views: usize,
}

// setup mongo server
pub async fn setup() -> Database {
    let adress = std::env::var("MONGO_URL")
        .ok()
        .and_then(|s| Some(s))
        .unwrap_or("mongodb://localhost:27017/".to_string());

    mongodb::Client::with_uri_str(adress)
        .await
        .expect("Can't connect to database")
        .database("ripgvc")
}
