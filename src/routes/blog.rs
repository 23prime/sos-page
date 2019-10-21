use rocket::get;
use rocket_contrib::templates::Template;

use super::{UnderConstContext, HRef};

#[get("/blog")]
pub fn blog() -> Template {
    let context = UnderConstContext {
        title: "Member Blog",
        img_path: "./images/under_construction.jpg",
        refs: vec![
            HRef {name: "Discography", link: "./discography"},
            HRef {name: "Member Blog", link: "./blog"},
        ]
    };

    return Template::render("blog", &context);
}
