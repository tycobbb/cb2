use rocket::State;
use rocket::response::content::Html;
use juniper_rocket::{ GraphQLRequest, GraphQLResponse };
use graphql::Schema;

// routes
#[get("/")]
pub fn graphiql() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get(
    request: GraphQLRequest,
    schema:  State<Schema>,
) -> GraphQLResponse {
    request.execute(&schema.0, &())
}

#[post("/graphql", data = "<request>")]
pub fn post(
    request: GraphQLRequest,
    schema:  State<Schema>,
) -> GraphQLResponse {
    request.execute(&schema.0, &())
}
