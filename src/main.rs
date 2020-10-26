#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod api;
mod databasing;
mod overview;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;


#[get("/")]
fn login_plain() -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("root".to_string(), ".".to_string());
    let template = "login";
    Template::render(template, &context)
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/api", routes![api::validate_token])
        .mount("/", routes![login_plain, overview::overview])
        .attach(Template::fairing())
        .launch();
}
