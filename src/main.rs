#![feature(proc_macro_hygiene, decl_macro)]

use juniper_rocket;
use rocket::{response::content, State};

mod schema;

use schema::*;

/// The GraphiQL Interface
#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

/// Handles GraphQL queries where
/// the query is part of the request body
#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<()>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(())
        .manage(Schema::new(Query, Mutation::new()))
        .mount("/", rocket::routes![graphiql, post_graphql_handler])
        .launch();
}
