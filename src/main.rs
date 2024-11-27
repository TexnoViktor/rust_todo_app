#[macro_use] extern crate rocket;

mod models;
mod storage;
mod routes;

use rocket::fs::{FileServer, relative};
use crate::storage::TaskStorage;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let storage = TaskStorage::new("tasks.json");

    rocket::build()
        .manage(storage)
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![
            routes::get_tasks,
            routes::create_task,
            routes::update_task,
            routes::delete_task
        ])
        .launch()
        .await?;

    Ok(())
}