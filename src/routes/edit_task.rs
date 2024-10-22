use crate::routes::jwt_middleware::validator;
use actix_web::{web, HttpResponse, Responder};
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

#[derive(Debug, Deserialize)]
struct Task {
    task: String,
}

#[actix_web::put("/edit/{id}")]
pub async fn edit_task_(
    path: web::Path<PathInfo>,
    mongo_client: web::Data<Client>,
    updated_task: web::Json<Task>,
    auth: BearerAuth,
) -> impl Responder {
    match validator(&auth).await {
        Ok(claims) => {
            let _user_email = claims.sub;

            let collection: Collection<Task> = mongo_client.database("task").collection("tasks");

            let obj_id = match ObjectId::parse_str(&path.id) {
                Ok(oid) => oid,
                Err(_) => return HttpResponse::BadRequest().json("Invalid ID format"),
            };

            let filter = doc! { "_id": obj_id };
            let update = doc! { "$set": { "task": &updated_task.task } };

            match collection.update_one(filter, update).await {
                Ok(result) => {
                    if result.matched_count == 1 {
                        HttpResponse::Ok().json("Task edited successfully")
                    } else {
                        HttpResponse::NotFound().json("Task not found in the list")
                    }
                }
                Err(err) => HttpResponse::InternalServerError()
                    .json(format!("Failed to edit task: {}", err)),
            }
        }
        Err(_) => {
            // Validation failed
            HttpResponse::Unauthorized().json("Invalid token")
        }
    }
}
