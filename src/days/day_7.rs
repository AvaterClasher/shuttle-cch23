use std::collections::HashMap;

use axum::routing::get;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use base64::Engine;

type Recipe = HashMap<String, usize>;

const COOKIE_NAME: &str = "recipe";

pub(super) fn route() -> Router {
    Router::new()
        .route("/decode", get(decode))
        .route("/bake", get(bake))
}

async fn decode(jar: CookieJar) -> Json<serde_json::Value> {
    decode_cookie(&jar).map(Json).unwrap()
}

#[derive(serde::Deserialize)]
struct BakeInput {
    recipe: Recipe,
    pantry: Recipe,
}

#[derive(serde::Serialize)]
struct BakeResult {
    cookies: usize,
    pantry: Recipe,
}

async fn bake(jar: CookieJar) -> Json<BakeResult> {
    decode_cookie(&jar)
        .map(|mut x: BakeInput| {
            let mut cookies = usize::MAX;
            // locate amount of cookies available to make
            for (ingredient, recipe_amount) in &x.recipe {
                if let Some(pantry_amount) = x.pantry.get(ingredient) {
                    cookies = (pantry_amount / recipe_amount).min(cookies);
                    if cookies == 0 {
                        // we can't make a single cookie if we don't have enough of one of the ingredients
                        return Json(BakeResult {
                            cookies,
                            pantry: x.pantry,
                        });
                    }
                } else {
                    // ingredient not found in the pantry? we can't make cookies
                    return Json(BakeResult {
                        cookies: 0,
                        pantry: x.pantry,
                    });
                }
            }
            // recalculate pantry based on the number of cookies to make
            for (ingredient, recipe_amount) in x.recipe {
                let amount_needed = recipe_amount * cookies;
                x.pantry
                    .entry(ingredient)
                    .and_modify(|x| *x -= amount_needed);
            }
            Json(BakeResult {
                cookies,
                pantry: x.pantry,
            })
        })
        .unwrap()
}

fn decode_cookie<T: serde::de::DeserializeOwned>(jar: &CookieJar) -> Option<T> {
    let recipe = jar.get(COOKIE_NAME)?;
    base64::engine::general_purpose::STANDARD
        .decode(recipe.value())
        .map(|x| serde_json::from_slice(&x).unwrap())
        .ok()
}
