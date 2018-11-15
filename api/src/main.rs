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
mod schema;
mod game;

use self::diesel::prelude::*;
use self::schema::*;

use rocket::State;
use rocket::response::content::Html;
use juniper::{ Context, FieldResult, RootNode, EmptyMutation };
use juniper_rocket::{ GraphQLRequest, GraphQLResponse };

use game::Game;

struct Root;

impl Root {
    fn new() -> Root {
        Root { }
    }
}

graphql_object!(Root: () |&self| {
    field game() -> FieldResult<Game> {
        let conn   = db::connect();
        let result = games::table
            .first::<Game>(&conn);

        match result {
            Ok(data) => Ok(data),
            Err(err) => Err(err)?
        }
    }
});

type Schema = RootNode<'static, Root, EmptyMutation<()>>;

// graphql
#[get("/")]
fn graphiql() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn graphql_get(
    request: GraphQLRequest,
    schema:  State<Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &())
}

#[post("/graphql", data = "<request>")]
fn graphql_post(
    request: GraphQLRequest,
    schema:  State<Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &())
}

// bootstrap
fn main() {
    dotenv::dotenv()
        .ok();

    rocket::ignite()
        .manage(Schema::new(
            Root::new(),
            EmptyMutation::<()>::new()
        ))
        .mount("/", routes![
            graphiql,
            graphql_get,
            graphql_post
        ])
        .launch();
}
