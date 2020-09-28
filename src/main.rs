#![feature(proc_macro_hygiene, decl_macro)]

use std::time::Instant;

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
    let start = Instant::now();
    let response = request.execute(&schema, &context);
    println!("Request took {:?}", start.elapsed());
    response
}

fn main() {
    rocket::ignite()
        .manage(DataContext::init().unwrap())
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
