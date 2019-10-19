use rocket::get;
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

mod counter;
use counter::*;

#[derive(Serialize)]
struct HRef {
    name: &'static str,
    link: &'static str
}

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
        refs: vec![
            HRef {name: "Discography", link: "./discography"},
            HRef {name: "Member Blog", link: "./blog"},
        ],

    };

    return Template::render("index", &context);
}
