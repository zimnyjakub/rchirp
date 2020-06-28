#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::database;
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::{State};
use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Post {
    text: String,
    author: String,
}

struct ExampleState {
    count: AtomicUsize
}

#[database("mongodb")]
struct MyDatabase(mongodb::db::Database);

#[get("/posts")]
fn all_posts(ex_state: State<ExampleState>) -> String {
    let v = ex_state.count.load(Ordering::Relaxed);
    format!("counter: {}", v)
}

#[post("/posts", format = "json", data = "<post>")]
fn save_post<'a>(post: Json<Post>) -> String {
    String::from(&post.text)
}

fn main() {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from("mongodb://root:rootpassword@localhost:27017/admin"));
    databases.insert("mongodb", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(3000)
        .extra("databases", databases)
        .finalize()
        .expect("failed to init config");

    rocket::custom(config)
        .attach(MyDatabase::fairing())
        .mount("/api/v1/", routes![all_posts, save_post])
        .launch();
}

