#![feature(proc_macro_hygiene, decl_macro)]
// to silence warnings when deriving diesel traits
// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

extern crate dotenv;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;
extern crate serde;

mod db;
mod schema;
mod game;

use self::diesel::prelude::*;
use self::schema::*;
use rocket_contrib::json::Json;

// response envelope
#[derive(Serialize)]
struct Resp<T> {
    data:  Option<T>,
    error: Option<String>,
}

impl<T> Resp<T> {
    fn data(value: T) -> Resp<T> {
        Resp { data: Some(value), error: None }
    }

    fn error(err: String) -> Resp<T> {
        Resp { data: None, error: Some(err) }
    }

    fn from(result: QueryResult<T>) -> Resp<T> {
        match result {
            Ok(data) => Resp::data(data),
            Err(err) => Resp::error(err.to_string())
        }
    }
}

// games
use self::game::{ Game, NewGame };

#[get("/game")]
fn read_game() -> Json<Resp<Game>> {
    let conn   = db::connect();
    let result = games::table
        .first::<Game>(&conn);

    Json(Resp::from(result))
}

#[post("/games/create")]
fn create_game() -> Json<Resp<Game>> {
    let new_game = NewGame {
        name: "Hanabi"
    };

    let conn   = db::connect();
    let result = diesel::insert_into(games::table)
        .values(&new_game)
        .get_result::<Game>(&conn);

    Json(Resp::from(result))
}

fn main() {
    dotenv::dotenv().ok();
    rocket::ignite().mount("/", routes![read_game, create_game]).launch();
}
