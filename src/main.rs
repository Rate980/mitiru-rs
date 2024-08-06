mod mitiru;

use std::{fs, io::BufReader};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use axum_macros::debug_handler;
use mitiru::{QuoteContent, Quotes};
use serde::{Deserialize, Serialize};

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
async fn get_content(
    Path(path): Path<String>,
    State(quotes): State<Quotes>,
) -> Result<Json<ContentTypes>, StatusCode> {
    // println!("{}", path);
    let path = path.split("/").collect::<Vec<&str>>();
    if path.first().is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let path = if path.last().unwrap().is_empty() {
        let mut path = path;
        path.pop();
        path
    } else {
        path
    };

    match *path.first().unwrap() {
        "title" => Ok(Json(ContentTypes::String(quotes.title.clone()))),
        "postscript" => Ok(Json(ContentTypes::String(quotes.postscript.clone()))),
        "main" => {
            if path.get(1).is_none() {
                Ok(Json(ContentTypes::Main(quotes.main.clone())))
            } else {
                let index = path.get(1).unwrap().parse::<usize>().unwrap();
                if path.get(2).is_none() {
                    Ok(Json(ContentTypes::QuoteContent(quotes.main[index].clone())))
                } else {
                    let key = path.get(2).unwrap();
                    match *key {
                        "character" => Ok(Json(ContentTypes::String(
                            quotes.main[index].character.clone(),
                        ))),
                        "quote" => Ok(Json(ContentTypes::String(quotes.main[index].quote.clone()))),
                        "story" => Ok(Json(ContentTypes::String(quotes.main[index].story.clone()))),
                        "how_to_use" => {
                            if path.get(3).is_none() {
                                Ok(Json(ContentTypes::VecString(
                                    quotes.main[index].how_to_use.clone(),
                                )))
                            } else if let Ok(use_index) = path.get(3).unwrap().parse::<usize>() {
                                Ok(Json(ContentTypes::String(
                                    quotes.main[index].how_to_use[use_index].clone(),
                                )))
                            } else {
                                Err(StatusCode::NOT_FOUND)
                            }
                        }
                        _ => Err(StatusCode::NOT_FOUND),
                    }
                }
            }
        }
        _ => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(fs::File::open("mitiru.json").unwrap());
    let quotes: mitiru::Quotes = serde_json::from_reader(&mut reader).unwrap();

    // let root_handler = || async { quotes };
    let app = Router::new()
        .route("/", get(rote_handler).with_state(quotes.clone()))
        .route("/*key", get(get_content).with_state(quotes));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
