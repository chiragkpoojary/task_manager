use crate::routes::jwt_middleware::validator;
use actix_web::{delete, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct PathInfo {
    id: String,
}

#[derive(Deserialize)]
struct Task {
    task: String,
}

#[delete("/delete/{id}")]
pub async fn delete_task_(
    path: web::Path<PathInfo>,
    mongo_client: web::Data<Client>,
    auth: BearerAuth,
) -> impl Responder {
    match validator(&auth).await {
        Ok(claims) => {
            let obj_id = match ObjectId::parse_str(&path.id) {
                Ok(oid) => oid,
                Err(_) => return HttpResponse::BadRequest().json("Invalid ID format"),
            };

            let collection: Collection<Task> = mongo_client.database("task").collection("tasks");
            let filter = doc! { "_id": obj_id };

            match collection.delete_one(filter).await {
                Ok(result) => {
                    if result.deleted_count == 1 {
                        HttpResponse::Ok().json("Task deleted successfully")
                    } else {
                        HttpResponse::NotFound().json("Task not found")
                    }
                }
                Err(err) => HttpResponse::InternalServerError()
                    .json(format!("Failed to delete task: {}", err)),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid token"),
    }
}
