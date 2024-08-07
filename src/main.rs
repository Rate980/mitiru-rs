mod mitiru;

use core::panic;
use std::{fs, io::BufReader};

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use mitiru::{QuoteContent, Quotes};
use serde::{Deserialize, Serialize};
use tower_http::catch_panic::CatchPanicLayer;

async fn rote_handler(State(quotes): State<Quotes>) -> Json<Quotes> {
    Json(quotes.clone())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum ContentTypes {
    Main(Vec<QuoteContent>),
    String(String),
    VecString(Vec<String>),
    QuoteContent(QuoteContent),
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
async fn test() -> Json<Option<String>> {
    panic!("test");
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

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(fs::File::open("mitiru.json").unwrap());
    let quotes: mitiru::Quotes = serde_json::from_reader(&mut reader).unwrap();

    // let root_handler = || async { quotes };
    let app = Router::new()
        .route("/", get(rote_handler).with_state(quotes.clone()))
        .route("/title", get(get_title).with_state(quotes.clone()))
        .route(
            "/postscript/",
            get(get_postscript).with_state(quotes.clone()),
        )
        .route("/main", get(get_main).with_state(quotes.clone()))
        .route(
            "/main/:index",
            get(get_mein_content).with_state(quotes.clone()),
        )
        .route("/test", get(test))
        .layer(CatchPanicLayer::new());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
