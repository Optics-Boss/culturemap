use axum::{Json, Router, response::IntoResponse, routing::get};
mod models;

use models::worldheritage::WorldHeritage;
use models::worldheritagejson::WorldHeritageJson;

use tower_http::cors::{Any, CorsLayer};

async fn get_sites() -> impl IntoResponse {
    let mut reader = csv::Reader::from_path("src/assets/whc001.csv").expect("Cant read CSV file");
    let mut wh_sites : Vec<WorldHeritageJson> = Vec::new();

    for result in reader.deserialize() {
        let wh_site: WorldHeritage = result.expect("Cant deserialize");

        wh_sites.push(WorldHeritageJson {
            name: wh_site.name,
            description: wh_site.description,
            coordinates: wh_site.coordinates,
            region: wh_site.region,
            state: wh_site.state,
        });
    }

    Json(wh_sites)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_methods(Any);

    let app = Router::new()
            .route("/", get(get_sites))
            .layer(cors);

    println!("Listening on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

