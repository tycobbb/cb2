#![feature(proc_macro_hygiene, decl_macro)]
// to silence warnings when deriving diesel traits
// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod schema;
mod game;

use self::diesel::prelude::*;
use self::schema::*;

#[get("/games")]
fn games() -> String {
    use self::game::Game;

    let conn = db::connect();

    // load games
    let records = games::table.limit(1)
        .load::<Game>(&conn)
        .expect("Error Loading games");

    let record = &records[0];

    format!("found a game named {}!", record.name)
}

#[post("/games/add")]
fn add_game() -> &'static str {
    use self::game::{ Game, NewGame };

    let conn = db::connect();

    // add game
    let new_game = NewGame {
        name: "Hanabi"
    };

    diesel::insert_into(games::table)
        .values(&new_game)
        .get_result::<Game>(&conn)
        .expect("Error adding new game.");

    "Added game."
}

fn main() {
    dotenv::dotenv().ok();
    rocket::ignite().mount("/", routes![games, add_game]).launch();
}
