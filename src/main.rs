#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::database;
use serde::{Serialize, Deserialize};
use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use rocket_contrib::databases::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use rocket::response::{status, content};
use bson::Bson;

#[macro_use]
extern crate bson;

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    text: String,
    author: String,
}

#[database("mongodb")]
struct MyDatabase(mongodb::db::Database);

#[get("/posts")]
fn all_posts(conn: MyDatabase) -> Json<Vec<Post>> {
    let collection = conn.0.collection("posts");
    let mut cursor = collection.find(None, None).unwrap();
    let mut all = Vec::new();

    while let Some(result) = cursor.next() {
        match result {
            Ok(document) => {
                all.push(Post {
                    text: document.get("text").and_then(Bson::as_str).unwrap_or("").to_string(),
                    author: document.get("author").and_then(Bson::as_str).unwrap_or("").to_string(),
                })
            }
            Err(_) => {}
        }
    }
    Json(all)
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

