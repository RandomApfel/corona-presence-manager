#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
use rocket::http::Cookies;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use serde::Serialize;
use super::databasing;


fn failed_overview(error_message: String, delete_cookie: bool) -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    let template = "fail";
    context.insert("error_message".to_string(), error_message);
    context.insert("button_link".to_string(), "./".to_string());
    context.insert("button_text".to_string(), "Zurück zum Login".to_string());
    if delete_cookie {
        context.insert("delete_cookie".to_string(), "true".to_string());
    } else {
        context.insert("delete_cookie".to_string(), "false".to_string());
    }
    Template::render(template, &context)
}

#[derive(Serialize)]
struct OverviewData {
    root: String,  // where is the web root, e.g. ../
    admin: bool,
    full_name: String,
    clock_list: Vec<String>,
    user_presences: Vec<(String, String, String)>
}


#[get("/overview")]
pub fn overview(cookies: Cookies) -> Template {
    let token = cookies.get("corona-presence-token");
    // no token set
    if token.is_none() {
        return failed_overview("Nicht eingeloggt!".to_string(), true);
    }

    // token invalid
    let user_from_token = databasing::user_from_token(
        &token.unwrap().value().to_string()
    );
    if user_from_token.is_none() {
        return failed_overview("Hinterlegter token nicht in der Datenbank.".to_string(), true);
    }

    let database_user = user_from_token.unwrap();

    let mut clock_list = Vec::new();
    for i in 0..24 {
        let hour = i;
        let minute = 0;
        clock_list.push(format!("{:02}{:02}", hour, minute).to_string());
    }

    let mut user_presences = Vec::new();

    let overview_data = OverviewData {
        root: "..".to_string(),
        admin: database_user.admin,
        full_name: database_user.full_name,
        clock_list: clock_list,
        user_presences: user_presences,
    };
    let template = "overview";
    Template::render(template, &overview_data)
}
