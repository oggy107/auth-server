#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use warp::Filter;

fn ping() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| "Hello, World!")
}

#[derive(Clone, Serialize)]
enum Role {
    Admin,
    User,
}

#[derive(Clone, Serialize)]
struct User {
    uid: String,
    email: String,
    pwd: String,
    role: Role,
}

fn login_route() -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone
{
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler)
        .recover(handle_rejection)
}

#[derive(Debug)]
struct InvalidData;

impl warp::reject::Reject for InvalidData {}

async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            "NOT_FOUND",
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if let Some(e) = err.find::<InvalidData>() {
        Ok(warp::reply::with_status(
            "BAD_REQUEST",
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(warp::reply::with_status(
            "INTERNAL_SERVER_ERROR",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

async fn login_handler(body: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    if body.contains_key("email") && body.contains_key("pwd") {
        let user = User {
            uid: "123".to_string(),
            email: body.get("email").unwrap().to_string(),
            pwd: body.get("pwd").unwrap().to_string(),
            role: Role::User,
        };

        return Ok(warp::reply::json(&user));
    } else {
        return Err(warp::reject::custom(InvalidData));
    }
}

#[tokio::main]
async fn main() {
    // let ping = ping();
    let app = login_route();

    warp::serve(app).run(([127, 0, 0, 1], 3030)).await;
}
