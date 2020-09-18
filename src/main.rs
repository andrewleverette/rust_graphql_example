#![feature(proc_macro_hygiene, decl_macro)]

use juniper_rocket;
use rocket::{response::content, State};

mod db;
mod models;
mod schema;

use db::DataContext;
use schema::*;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/grapqhl?<request>")]
fn get_graphql_handler(
    context: State<DataContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<DataContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(DataContext::init().unwrap())
        .manage(Schema::new(Query, Mutation::new()))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
