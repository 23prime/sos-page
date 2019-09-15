#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]


// For DB
extern crate dotenv;
extern crate postgres;

use std::env;
use dotenv::dotenv;
use postgres::{Connection, TlsMode};


fn establis_connection() -> Connection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    return Connection::connect(db_url, TlsMode::None).unwrap();
}

fn get_counter(conn: &Connection) -> i32 {
    let q = "SELECT counter FROM access_counter;";
    let rows = conn.query(q, &[]).unwrap();
    let value: i32 = rows.get(0).get(0);
    return value
}

fn update_counter(conn: &Connection, num: i32) {
   let q = format!("UPDATE access_counter set counter = {}", num);
   conn.execute(&q, &[]).unwrap();
}


// For main
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_include_static_resources;


use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_include_static_resources::StaticResponse;

// mod counter;
// use counter::*;

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
    let conn = establis_connection();
    let mut counter = get_counter(&conn);
    counter += 1;
    update_counter(&conn, counter);

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
