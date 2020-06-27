#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/posts")]
fn all_posts() -> &'static str {
    "mordo"
}

#[post("/posts")]
fn save_post() -> &'static str {
    "mordo zapisuje"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

