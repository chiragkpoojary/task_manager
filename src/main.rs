use actix_web::web;
use mongodb::Client;
use dotenv::dotenv;
use std::env;
mod routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
    let mongo_client = Client::with_uri_str(&database_url)
        .await
        .expect("Failed to connect to MongoDB");

    let mongo_data = web::Data::new(mongo_client);

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(mongo_data.clone())
            .service(routes::sign_in)
            .service(routes::sign_up)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
