use rocket::{routes, catchers};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

mod index;
mod blog;
mod discography;
mod statics;
mod not_found;

#[derive(Serialize)]
pub struct HRef {
    name: &'static str,
    link: &'static str
}

type HRefs = [HRef; 2];

pub static REFS: HRefs = [
    HRef {name: "Discography", link: "./discography"},
    HRef {name: "Member Blog", link: "./blog"}
];

#[derive(Serialize)]
pub struct UnderConstContext {
    title: &'static str,
    img_path: &'static str,
    refs: &'static HRefs
}

#[derive(Serialize)]
struct IndexContext {
    counter: i32,
    refs: &'static HRefs
}

pub fn rocket() -> rocket::Rocket {
    let rts = routes![
        index::index,
        blog::blog,
        discography::discography,
        statics::files
    ];

    rocket::ignite()
        .mount("/", rts)
        .attach(Template::fairing())
        .register(catchers![not_found::not_found])
}
