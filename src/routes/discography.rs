use rocket::get;
use rocket_contrib::templates::Template;

use super::{UnderConstContext, HRef};

#[get("/discography")]
pub fn discography() -> Template {
    let context = UnderConstContext {
        title: "Discography",
        img_path: "./images/under_construction.jpg",
        refs: vec![
            HRef {name: "Discography", link: "./discography"},
            HRef {name: "Member Blog", link: "./blog"},
        ]
    };

    return Template::render("discography", &context);
}
