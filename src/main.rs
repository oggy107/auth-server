#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::sync::Arc;

mod route;

#[tokio::main]
async fn main() {
    // let users = Arc::new()
    let app = route::login();

    warp::serve(app).run(([127, 0, 0, 1], 3030)).await;
}
