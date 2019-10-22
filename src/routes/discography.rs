use rocket::get;
use rocket_contrib::templates::Template;

use super::{UnderConstContext, REFS};

#[get("/discography")]
pub fn discography() -> Template {
    let context = UnderConstContext {
        title: "Discography",
        img_path: "./images/under_construction.jpg",
        refs: &REFS
    };

    return Template::render("discography", &context);
}
