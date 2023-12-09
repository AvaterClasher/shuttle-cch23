use axum::Router;

pub mod day_1;
pub mod day_4;
pub mod day_6;
pub mod day_7;
pub mod day_8;

pub fn routes() -> Router {
    Router::new()
        .nest("/1", day_1::route())
        .nest("/4", day_4::route())
        .nest("/6", day_6::route())
        .nest("/7", day_7::route())
        .nest("/8", day_8::route())
}