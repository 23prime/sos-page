use rocket::get;
use rocket_contrib::templates::Template;

mod counter;
use counter::*;

use super::{IndexContext, REFS};

#[get("/")]
pub fn index() -> Template {
    let counter = count();
    let context = IndexContext {
        counter: counter,
        refs: &REFS
    };

    return Template::render("index", &context);
}
