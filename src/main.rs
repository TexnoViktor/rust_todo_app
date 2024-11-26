use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use serde_json;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

mod model;
mod storage;

use model::Task;
use storage::TaskStorage;

#[derive(Deserialize, Serialize)]
struct TaskIdParam {
    id: Uuid,
}

async fn get_tasks(storage: web::Data<TaskStorage>) -> impl Responder {
    match storage.load_tasks() {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().body("Не вдалося завантажити завдання"),
    }
}

async fn create_task(
    storage: web::Data<TaskStorage>,
    task_data: web::Json<Task>,
) -> impl Responder {
    let task = task_data.into_inner();
    match storage.add_task(task) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Не вдалося створити завдання"),
    }
}

async fn update_task(
    storage: web::Data<TaskStorage>,
    task_data: web::Json<Task>,
) -> impl Responder {
    let task = task_data.into_inner();
    match storage.update_task(task.id, task) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Не вдалося оновити завдання"),
    }
}

async fn delete_task(
    storage: web::Data<TaskStorage>,
    web::Path(param): web::Path<TaskIdParam>,
) -> impl Responder {
    match storage.delete_task(param.id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Не вдалося видалити завдання"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = web::Data::new(TaskStorage::new("tasks.json"));

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}