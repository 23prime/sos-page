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

#[derive(Serialize)]
pub struct UnderConstContext {
    title: &'static str,
    img_path: &'static str,
    refs: Vec<HRef>
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
