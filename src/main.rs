#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

mod route;
use route::rocket;

fn main() {
    rocket().launch();
}
