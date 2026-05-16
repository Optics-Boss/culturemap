use axum::{Json, Router, response::IntoResponse, routing::get};
mod models;

use models::worldheritage::WorldHeritage;

async fn get_sites() -> impl IntoResponse {
    let mut reader = csv::Reader::from_path("src/assets/whc001.csv").expect("Cant read CSV file");
    let mut _wh_sites : Vec<WorldHeritage> = Vec::new();

    for result in reader.deserialize() {
        let wh_site: WorldHeritage = result.expect("Cant deserialize");
        _wh_sites.push(wh_site);
    }

    Json(_wh_sites)
}

#[tokio::main]
async fn main() {
     let app = Router::new()
            .route("/", get(get_sites));

    println!("Listening on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

