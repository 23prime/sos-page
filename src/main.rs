#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

mod routes;
use routes::rocket;

fn main() {
    rocket().launch();
}
