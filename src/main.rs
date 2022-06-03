#[macro_use]
extern crate rocket;

use std::sync::Arc;

use rocket::serde::json::Json;
use rocket::State;
use service::system_monitoring;
use tokio::{select, sync::RwLock};

mod entity;
mod service;

#[get("/")]
async fn status() -> &'static str {
    "ok"
}

type SystemInformation = Arc<RwLock<entity::system::SystemData>>;

#[get("/system", format = "json")]
async fn get_temps(system_data: &State<SystemInformation>) -> Json<entity::system::SystemData> {
    let value = system_data.read().await;
    Json(value.clone())
}

#[rocket::main]
async fn main() {
    let value = SystemInformation::new(RwLock::new(entity::system::SystemData::default()));

    let (send, recv) = tokio::sync::oneshot::channel();

    let updater = system_monitoring::update_value(value.clone(), recv);
    let server = rocket::build()
        .manage(value.clone())
        .mount("/", routes![status, get_temps])
        .launch();

    let _result = select! {
        _ = updater => {},
        _ = server => {
            send.send(true).unwrap();
            println!("Server stopped");
        },
    };
}
