#![feature(plugin)]
#![plugin(rocket_codegen)]

// to silence warnings when deriving diesel traits
// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate juniper_codegen;
extern crate juniper_rocket;

mod db;
mod game;
mod graphql;

// bootstrap
fn main() {
    dotenv::dotenv()
        .ok();

    rocket::ignite()
        .manage(
            graphql::Schema::new()
        )
        .mount("/", routes![
            graphql::graphiql,
            graphql::get,
            graphql::post,
        ])
        .launch();
}
