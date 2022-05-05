#[macro_use] extern crate rocket;

#[get("/")]
async fn status() -> &'static str {
    "status"
}

#[rocket::main]
async fn main(){
    let _ = rocket::build()
    .mount("/", routes![status])
    .launch().await;
}
