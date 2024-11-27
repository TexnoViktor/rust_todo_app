use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;
use crate::models::Task;
use crate::storage::TaskStorage;

#[rocket::get("/tasks")]
pub fn get_tasks(storage: &State<TaskStorage>) -> Json<Vec<Task>> {
    Json(storage.load_tasks().unwrap_or_default())
}

#[rocket::post("/tasks", format = "json", data = "<task>")]
pub fn create_task(storage: &State<TaskStorage>, task: Json<Task>) -> rocket::http::Status {
    match storage.add_task(task.into_inner()) {
        Ok(_) => rocket::http::Status::Created,
        Err(_) => rocket::http::Status::InternalServerError,
    }
}

#[rocket::put("/tasks/<id>", format = "json", data = "<task>")]
pub fn update_task(storage: &State<TaskStorage>, id: String, task: Json<Task>) -> rocket::http::Status {
    let task_id = Uuid::parse_str(&id).expect("Invalid UUID");
    match storage.update_task(task_id, task.into_inner()) {
        Ok(_) => rocket::http::Status::Ok,
        Err(_) => rocket::http::Status::InternalServerError,
    }
}

#[rocket::delete("/tasks/<id>")]
pub fn delete_task(storage: &State<TaskStorage>, id: String) -> rocket::http::Status {
    let task_id = Uuid::parse_str(&id).expect("Invalid UUID");
    match storage.remove_task(task_id) {
        Ok(_) => rocket::http::Status::Ok,
        Err(_) => rocket::http::Status::InternalServerError,
    }
}