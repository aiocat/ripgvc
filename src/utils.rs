// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// check github username to see if it exists
pub async fn check_github_username(username: &str) -> bool {
    let client = reqwest::Client::builder();

    let response = client
        .use_rustls_tls()
        .build()
        .expect("Failed to build HTTP Client")
        .get(format!("https://github.com/{}", username))
        .send()
        .await
        .expect("HTTP Request error");

    response.status() == 200
}

// check hex color
pub fn check_hex(color: &str) -> bool {
    if color.len() != 6 {
        return false;
    }

    for character in color.chars() {
        if !character.is_ascii_hexdigit() {
            return false;
        }
    }

    true
}
