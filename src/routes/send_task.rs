use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

use futures_util::stream::StreamExt;


#[derive(Debug, Serialize, Deserialize)]
struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,  // Keep MongoDB's default _id field
    task: String,          // Store the task name separately
}

#[get("/tasks")]
pub async fn get_tasks(
     mongo_client: web::Data<Client>,
    //key: web::Data<HS256Key>,
) -> impl Responder {

    //// Extract and verify the access token
    //let token = req.headers().get("Authorization").and_then(|header| {
    //    header.to_str().ok().map(|s| s.trim_start_matches("Bearer "))
    //});

    //if let Some(token) = token {
    //    if let Ok(claims) = key.verify_token::<NoCustomClaims>(token, None) {
            // Extract the user's email from the token claims
            //let user_email = claims.subject.unwrap_or_default();

            // Access the MongoDB collection
            let collection: Collection<Task> = mongo_client.database("task").collection("tasks");
            let filter = doc! {};

    //        // Find tasks for the logged-in user
    //        let mut cursor = match collection.find(filter).await {
    //            Ok(cursor) => cursor,
    //            Err(e) => return HttpResponse::InternalServerError().json(format!("Failed to fetch tasks: {}", e)),
    //        };
    //
    //        let mut tasks = Vec::new();
    //        while let Some(result) = cursor.next().await {
    //            match result {
    //                Ok(task) => tasks.push(task),
    //                Err(e) => return HttpResponse::InternalServerError().json(format!("Failed to fetch task: {}", e)),
    //            }
    //        }
    //
    //        HttpResponse::Ok().json(tasks)
    ////    } else {
    ////        HttpResponse::Unauthorized().json("Invalid token")
    ////    }
    ////} else {
    ////    HttpResponse::Unauthorized().json("Authorization token is missing")
    ////}
//}
match collection.find(filter).await {
        Ok(mut cursor) => {
            let mut tasks = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(task) => tasks.push(task),
                    Err(e) => return HttpResponse::InternalServerError().json(format!("Failed to fetch task: {}", e)),
                }
            }
            HttpResponse::Ok().json(tasks)
        },
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().json(format!("Failed to fetch tasks: {}", e))
        },
    }
}
