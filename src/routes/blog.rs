use rocket::get;
use rocket_contrib::templates::Template;

use super::{UnderConstContext, REFS};

#[get("/blog")]
pub fn blog() -> Template {
    let context = UnderConstContext {
        title: "Member Blog",
        img_path: "./images/under_construction.jpg",
        refs: &REFS
    };

    return Template::render("blog", &context);
}
