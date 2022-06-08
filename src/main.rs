use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use mongodb::{bson::doc, options::ReplaceOptions, Collection};
use std::collections::HashMap;
use std::net::SocketAddr;

mod database;
mod drawing;
mod utils;

#[tokio::main]
async fn main() {
    // connect to the database
    if let Err(error) = database::setup().await {
        panic!("{}", error);
    }

    // new app router
    let app = Router::new().route("/", get(root));

    // get port
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    // start listening
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn set_response_template(value: String) -> impl IntoResponse {
    ([("content-type", "image/svg+xml"), ("cache-control", "max-age=0, no-cache, no-store, must-revalidate")], value)
}

// basic handler that responds with a static string
async fn root(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let color = match params.get("color") {
        Some(color) => {
            if utils::check_hex(color) {
                format!("#{}", color)
            } else {
                String::from("#42a5f5")
            }
        }
        None => String::from("#42a5f5"),
    };

    match params.get("username") {
        Some(username) => {
            let connection = database::get_connection();
            let users: Collection<database::User> = connection.collection("users");

            // check if exists in database
            let result = users
                .find_one(doc! {"_id": username}, None)
                .await
                .expect("Database error");
            if let Some(mut user) = result {
                // user found
                let mut replace_option = ReplaceOptions::default();
                replace_option.upsert = Some(true);

                // increase
                let new_value = user.views + 1;
                user.views = new_value;
                users
                    .replace_one(doc! {"_id": username}, user, replace_option)
                    .await
                    .expect("Database error");

                // return new
                set_response_template(drawing::draw_file(&new_value.to_string(), color))
            } else {
                // check if github account exists
                if utils::check_github_username(&username).await {
                    // create new user
                    let user = database::User {
                        _id: username.clone(),
                        views: 0,
                    };
                    users.insert_one(user, None).await.expect("Database error");
                }
                set_response_template(drawing::draw_file("0", color))
            }
        }
        None => set_response_template(drawing::draw_file(
            "User not found",
            String::from("#ff1744"),
        )),
    }
}
