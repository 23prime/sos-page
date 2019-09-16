#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_include_static_resources;


use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_include_static_resources::StaticResponse;

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

#[derive(Serialize)]
struct UnderConstContext {
    title: &'static str,
    img_path: &'static str
}

#[get("/")]
fn root() -> Template {
    let counter = count();
    let context = TemplateContext {
        counter: counter,
        refs: vec![
            HRef {name: "Discography", link: "./discography"},
            HRef {name: "Member Blog", link: "./blog"},
        ],

    };

    return Template::render("root", &context);
}

#[get("/favicon.ico")]
fn favicon() -> StaticResponse {
   static_response!("favicon")
}

#[get("/images/under_construction.jpg")]
fn under_construction_jpg() -> StaticResponse {
   static_response!("under_construction_jpg")
}

#[get("/CSS/under_construction.css")]
fn under_construction_css() -> StaticResponse {
   static_response!("under_construction_css")
}

#[get("/blog")]
fn blog() -> Template {
    let context = UnderConstContext {
        title: "Member Blog",
        img_path: "./images/under_construction.jpg"
    };

    return Template::render("blog", &context);
}

#[get("/discography")]
fn discography() -> Template {
    let context = UnderConstContext {
        title: "Discography",
        img_path: "./images/under_construction.jpg"
    };

    return Template::render("discography", &context);
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    return Template::render("error/404", &map);
}

fn rocket() -> rocket::Rocket {
    let rts = routes![
        favicon,
        root,
        blog,
        discography,
        under_construction_jpg,
        under_construction_css
    ];

    rocket::ignite()
        .mount("/", rts)
        .attach(Template::fairing())
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "favicon", "/favicon.ico",
                "under_construction_jpg", "/images/under_construction.jpg",
                "under_construction_css", "/CSS/under_construction.css"
            );
        }))
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
