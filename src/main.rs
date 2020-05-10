#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use argon2::{self, Config};
use postgres::{Client, NoTls};
use rocket::http::Method;
use rocket_contrib::json::Json;
use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use uuid::Uuid;

//fn save(db: &Database) {
//fs::write("db.json", serde_json::to_string(&db).unwrap()).unwrap();
//}
#[derive(Serialize, Deserialize, Debug)]
struct CheckToken {
    token: String,
}
/*#[derive(Deserialize, Serialize)]
struct Database {
    users: Mutex<HashMap<String, User>>,
    salt: String,
}*/

#[get("/")]
fn index() -> &'static str {
    "online"
}
#[post("/check_token", format = "application/json", data = "<data>")]
fn check_token(data: Json<CheckToken>) -> Json<serde_json::Value> {
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    if let Ok(user) = client.query_one(
        "SELECT username FROM tokens WHERE token = $1",
        &[&data.token],
    ) {
        if data.token != "" {
            Json(serde_json::json!({
                "error": false,
                "username": user.get::<&str, String>("username"),
            }))
        } else {
            Json(serde_json::json!({
                "error": true, "error_message": "Incorrect token"
            }))
        }
    } else {
        Json(serde_json::json!({
            "error": true, "error_message": "Incorrect token"
        }))
    }
}
fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "https://scratchyone.com",
        "https://www.scratchyone.com",
    ]);

    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn main() {
    thread::sleep(time::Duration::from_millis(2000));
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    let cfg = rocket::config::Config::build(rocket::config::Environment::Development)
        .port(80)
        .address("0.0.0.0")
        .unwrap();
    rocket::custom(cfg)
        .attach(make_cors())
        .mount("/", routes![index, check_token])
        .launch();
}
