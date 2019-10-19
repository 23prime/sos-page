use std::collections::HashMap;

use rocket::{catch, Request};
use rocket_contrib::templates::Template;

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    return Template::render("error/404", &map);
}

