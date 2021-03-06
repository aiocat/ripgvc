use axum::{
    async_trait,
    extract::{Extension, FromRequest, Query, RequestParts},
    response::IntoResponse,
    routing::get,
    Router,
};
use mongodb::{bson::doc, options::ReplaceOptions, Collection};
use std::collections::HashMap;
use std::net::SocketAddr;

mod database;
mod drawing;
mod utils;

struct HasUserAgent(bool);

#[async_trait]
impl<B> FromRequest<B> for HasUserAgent
where
    B: Send,
{
    type Rejection = &'static str;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();

        if let Some(user_agent) = headers.get("user-agent") {
            if user_agent
                .to_str()
                .expect("User-agent parse error")
                .contains("github-camo")
            {
                Ok(HasUserAgent(true))
            } else {
                Ok(HasUserAgent(false))
            }
        } else {
            Err("Invalid user-agent")
        }
    }
}

#[tokio::main]
async fn main() {
    // connect to the database
    let database: Collection<database::User> = database::setup().await.collection("users");

    // new app router
    let app = Router::new()
        .route("/", get(root))
        .layer(Extension(database));

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
    (
        [
            ("content-type", "image/svg+xml"),
            (
                "cache-control",
                "max-age=0, no-cache, no-store, must-revalidate",
            ),
        ],
        value,
    )
}

// main handler
async fn root(
    Extension(database): Extension<Collection<database::User>>,
    Query(params): Query<HashMap<String, String>>,
    HasUserAgent(valid_user_agent): HasUserAgent,
) -> impl IntoResponse {
    // get round mode
    let round = params.get("round").is_some();

    // get color code
    let color = match params.get("color") {
        Some(color) => {
            if utils::check_hex(color) {
                format!("#{}", color)
            } else {
                String::from("#1b5acf")
            }
        }
        None => String::from("#1b5acf"),
    };

    // get username
    match params.get("username") {
        Some(username) => {
            // check if exists in database
            let result = database
                .find_one(doc! {"_id": username}, None)
                .await
                .expect("Database error");
                
            if let Some(mut user) = result {
                // user found
                let mut replace_option = ReplaceOptions::default();
                replace_option.upsert = Some(true);

                // check user-agent
                if !valid_user_agent {
                    return set_response_template(drawing::draw_file(
                        &user.views.to_string(),
                        color,
                        round,
                    ));
                }

                // increase
                let new_value = user.views + 1;
                user.views = new_value;
                database
                    .replace_one(doc! {"_id": username}, user, replace_option)
                    .await
                    .expect("Database error");

                // return new
                set_response_template(drawing::draw_file(&new_value.to_string(), color, round))
            } else {
                // check if github account exists
                if utils::check_github_username(&username).await {
                    // create new user
                    let user = database::User {
                        _id: username.to_string(),
                        views: 0,
                    };
                    database
                        .insert_one(user, None)
                        .await
                        .expect("Database error");
                }
                set_response_template(drawing::draw_file("0", color, round))
            }
        }
        None => set_response_template(drawing::draw_file(
            "User not found",
            String::from("#1b5acf"),
            round,
        )),
    }
}
