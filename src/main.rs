#[macro_use] extern crate rocket;

use std::sync::Arc;

use rocket::serde::json::Json;
use service::system_monitoring;
use tokio::{sync::RwLock, join};
use rocket::State;

mod service;
mod entity;

#[get("/")]
async fn status() -> &'static str {
    "ok"
}

type SystemInformation = Arc<RwLock<entity::cpu::SystemData>>;

#[get("/cpu", format="json")]
async fn get_temps(system_data: &State<SystemInformation>) -> Json<entity::cpu::SystemData> {
    let value = system_data.read().await;
    Json(value.clone())
}

#[rocket::main]
async fn main(){
    let value = SystemInformation::new(RwLock::new(service::system_monitoring::get_current_value()));

    let updater = system_monitoring::update_value(value.clone());
    let server = rocket::build()
    .manage(value.clone())
    .mount("/", routes![status, get_temps])
    .launch();

    let _result = join!(server, updater);
}
