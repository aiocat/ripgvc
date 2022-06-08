// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Check github username to see if it exists
pub async fn check_username(username: &str) -> bool {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://github.com/{}", username))
        .send()
        .await
        .expect("HTTP Request error");

    response.status() == 200
}
