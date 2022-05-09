#[macro_use]
extern crate rocket;

use std::sync::Arc;

use rocket::serde::json::Json;
use rocket::State;
use service::system_monitoring;
use tokio::{join, sync::RwLock};

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

    let updater = system_monitoring::update_value(value.clone());
    let server = rocket::build()
        .manage(value.clone())
        .mount("/", routes![status, get_temps])
        .launch();

    let _result = join!(server, updater);
}
