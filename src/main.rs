#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::request::Form;


#[derive(FromForm)]
struct User {
    name: String,
    account: usize,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/item?<id>&<user..>")]
fn item(id: usize, user: Form<User>) {

}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}