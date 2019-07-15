#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http;
use std::path;
use rocket::response;
use rocket_contrib::serve;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/whats-up")]
fn whats_up() -> &'static str {
    "What's up fool!"
} 

#[get("/hello/<name>")]
fn greet(name: &http::RawStr) -> String {
    format!("Hello {}", name.as_str())
}

#[get("/yo/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're cool as a cucumber {} and only {} years old! Wowoweewar", name, age)
    } else {
        format!("Bah pathetic. Go home {} you're only {} years old. Pathetic worm", name, age)
    }
}

#[get("/files/<file..>")]
fn files(file: path::PathBuf) -> Option<response::NamedFile> {
    response::NamedFile::open(path::Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, whats_up, greet, hello, files])
        .mount("/public", serve::StaticFiles::from("static"))
        .launch();
}