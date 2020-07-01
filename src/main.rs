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
use std::convert::Infallible;
use rocket_contrib::databases::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use rocket::response::{status, content};

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;
#[macro_use(bson, doc)]
extern crate bson;

#[derive(Serialize, Deserialize, Debug)]
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

#[get("/posts/<id>")]
fn one_post(id: usize, conn: MyDatabase) -> Json<Post> {
    let post = Post { text: id.to_string(), author: "a".parse().unwrap() };
    Json(post)
}

#[post("/posts", format = "json", data = "<post>")]
fn save_post(post: Json<Post>, conn: MyDatabase) -> status::Accepted<()>{
    let collection = conn.0.collection("posts");
    let doc = doc! {"author":&post.author, "text": &post.text};
    collection.insert_one(doc, None).unwrap();

    status::Accepted::<()>(None)
}

fn main() {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from("mongodb://one:two@localhost:27017/one"));
    databases.insert("mongodb", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(3000)
        .extra("databases", databases)
        .finalize()
        .expect("failed to init config");

    rocket::custom(config)
        .attach(MyDatabase::fairing())
        .mount("/api/v1/", routes![one_post, all_posts, save_post])
        .launch();
}

