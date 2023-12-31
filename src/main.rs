use axum::http::StatusCode;
use axum::{routing::get, Router};

mod days;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_err() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_err))
        .nest("/", days::routes())
        .merge(days::day_6::get_routes());

    Ok(router.into())
}