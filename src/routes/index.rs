use rocket::get;
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

mod counter;
use counter::*;

// use super::{HRef, REFS};
use super::HRef;

#[derive(Serialize)]
struct TemplateContext {
    counter: i32,
    refs: Vec<HRef>
}

#[get("/")]
pub fn index() -> Template {
    let counter = count();
    let context = TemplateContext {
        counter: counter,
        // refs: REFS
        refs: vec![
            HRef {name: "Discography", link: "./discography"},
            HRef {name: "Member Blog", link: "./blog"},
        ],
    };

    return Template::render("index", &context);
}
