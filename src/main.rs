use actix_web::{web,HttpResponse,Responder,get};
use mongodb::Client;
use dotenv::dotenv;
use std::env;
mod routes;
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Task Manager API")
}

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
            // .service(routes::sign_in)
            // .service(routes::sign_up)
            .service(routes::addtask)
            .service(routes::get_tasks)
            .service(routes::delete_task_)
            .service(routes::edit_task_)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
