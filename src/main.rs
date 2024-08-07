mod mitiru;
mod quates;

use std::{fs, io::BufReader};

use axum::Router;
use mitiru::Quotes;
use quates::make_router;
use tower_http::catch_panic::CatchPanicLayer;

#[tokio::main]
async fn main() {
    // let mut reader = BufReader::new(fs::File::open("mitiru.json").unwrap());
    let quotes_dir = fs::read_dir("quotes").unwrap();

    // let root_handler = || async { quotes };
    let mut app = Router::new().layer(CatchPanicLayer::new());

    let mut quotes_list = Vec::new();
    for entry in quotes_dir.flatten() {
        if entry.file_type().unwrap().is_dir() {
            continue;
        }
        let path = entry.path();
        let extension = path.extension();
        if extension.is_none() {
            continue;
        }
        if extension.unwrap() != "json" {
            continue;
        }
        let name = path.file_stem().unwrap().to_str().unwrap();
        quotes_list.push(name.to_string());

        let quotes: Quotes =
            serde_json::from_reader(BufReader::new(fs::File::open(path.clone()).unwrap())).unwrap();
        app = app.nest(&format!("/{}", name), make_router(quotes));
    }
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
