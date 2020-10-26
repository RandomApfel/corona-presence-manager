use rusqlite::{params, Connection, Result, Row};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;

struct User {
    pub token: String,
    pub full_name: String,
}

struct Presence {
    pub quarter_begin: i32,
    pub quarter_end: i32,
    pub token: String,
}

fn establish_connection() -> Connection {
    Connection::open("db.sqlite3").unwrap()
}

pub fn is_token_valid(token: &String) -> bool {
    let con = establish_connection();

    println!("Check existance token {}", token);
    let count: i32 = con.query_row(
        "SELECT count(*) FROM users WHERE token = ?1",
        params![token],
        |row| row.get(0),
    )
    .unwrap();

    println!("Count somehow {}", count);

    return count > 0;
}

pub struct DatabaseUser {
    pub admin: bool,
    pub full_name: String,
}

pub fn user_from_token(token: &String) -> Option<DatabaseUser> {
    let con = establish_connection();

    let row: Result<(Result<String>, Result<bool>)> = con.query_row(
        "SELECT full_name, admin_view FROM users WHERE token = ?1",
        params![token],
        |row| Ok((row.get(0), row.get(1)))
    );

    let values = row.unwrap();

    if let Ok(full_name) = values.0 {  // name exists
        return Some(
            DatabaseUser {
                full_name: full_name,
                admin: values.1.unwrap_or(false)
            })
    }
    None
}

pub fn get_presence_count_at(unix_timestamp: u64) -> u16 {
    let con = establish_connection();

    // Konvertiere unix timestamp zu quarter, runde ab
    let quarter: i32 = (unix_timestamp / 900) as i32;

    let row: Result<i32> = con.query_row(
        "SELECT count(*) FROM presences WHERE quarter = ?1",
        params![quarter],
        |row| row.get(0)
    );

    row.unwrap() as u16
}
