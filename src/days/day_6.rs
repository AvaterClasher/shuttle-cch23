use axum::routing::post;
use axum::{Json, Router};

pub(super) fn route() -> Router {
    Router::new().route("/", post(elf_count))
}

#[derive(serde::Serialize)]
struct Elf {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn elf_count(body: String) -> Json<Elf> {
    let elf_on_a_shelf = body.matches("elf on a shelf").count();
    let shelves = body.matches("shelf").count();
    Json(Elf {
        elf: body.matches("elf").count(),
        elf_on_a_shelf,
        shelf_with_no_elf_on_it: shelves - elf_on_a_shelf,
    })
}