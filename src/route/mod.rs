use std::collections::HashMap;
use warp::Filter;

mod error;
mod modal;

pub fn login() -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone
{
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler)
        .recover(error::handle_rejection)
}

async fn login_handler(body: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    if body.contains_key("email") && body.contains_key("pwd") {
        let user = modal::User {
            uid: "123".to_string(),
            email: body.get("email").unwrap().to_string(),
            pwd: body.get("pwd").unwrap().to_string(),
            role: modal::Role::User,
        };

        return Ok(warp::reply::json(&user));
    } else {
        return Err(warp::reject::custom(error::InvalidData));
    }
}
