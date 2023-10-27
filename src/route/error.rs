#[derive(Debug)]
pub struct InvalidData;

impl warp::reject::Reject for InvalidData {}

pub async fn handle_rejection(
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
