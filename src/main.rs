<<<<<<< HEAD
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    docker::main().await;
}

=======
fn main() {
    println!("Hello, world!");
}
>>>>>>> 7a6736e8d47810000262ec341d213a5a88afeecf
