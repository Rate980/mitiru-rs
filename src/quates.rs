use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use rand::Rng;

use crate::mitiru::{QuoteContent, Quotes};

async fn rote_handler(State(quotes): State<Quotes>) -> Json<Quotes> {
    Json(quotes.clone())
}

#[debug_handler]
async fn get_title(State(quotes): State<Quotes>) -> Json<String> {
    Json(quotes.title.clone())
}

#[debug_handler]
async fn get_postscript(State(quotes): State<Quotes>) -> Json<String> {
    Json(quotes.postscript.clone())
}

#[debug_handler]
async fn get_main(State(quotes): State<Quotes>) -> Json<Vec<QuoteContent>> {
    Json(quotes.main.clone())
}

#[debug_handler]
async fn get_mein_content(
    Path(index): Path<usize>,
    State(quotes): State<Quotes>,
) -> Result<Json<QuoteContent>, StatusCode> {
    Ok(Json(
        quotes.main.get(index).ok_or(StatusCode::NOT_FOUND)?.clone(),
    ))
}

#[debug_handler]
async fn get_random(State(quotes): State<Quotes>) -> Json<QuoteContent> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..quotes.main.len());
    Json(quotes.main.get(index).unwrap().clone())
}

async fn get_character(
    Path(character): Path<String>,
    State(quotes): State<Quotes>,
) -> Json<Vec<QuoteContent>> {
    let mut result = Vec::new();
    for content in quotes.main.iter() {
        if content.character == character {
            result.push(content.clone());
        }
    }
    Json(result)
}

pub fn make_router(quotes: Quotes) -> Router {
    Router::new()
        .route("/", get(rote_handler).with_state(quotes.clone()))
        .route("/title", get(get_title).with_state(quotes.clone()))
        .route(
            "/postscript/",
            get(get_postscript).with_state(quotes.clone()),
        )
        .route("/main", get(get_main).with_state(quotes.clone()))
        .route("/main/random", get(get_random).with_state(quotes.clone()))
        .route(
            "/main/char/:character",
            get(get_character).with_state(quotes.clone()),
        )
        .route(
            "/main/:index",
            get(get_mein_content).with_state(quotes.clone()),
        )
}
