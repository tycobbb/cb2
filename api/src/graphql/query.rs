use diesel::prelude::*;
use juniper::FieldResult;
use db;
use db::schema::*;
use game::Game;

pub struct Query;

impl Query {
    pub fn new() -> Query {
        Query { }
    }
}

graphql_object!(Query: () |&self| {
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
