use juniper::{self, Context, EmptyMutation, RootNode};

use crate::db::DataContext;
use crate::models::*;

impl Context for DataContext {}

pub struct Query;

#[juniper::object(Context = DataContext)]
/// An example GraphQL Schema built with Rust
impl Query {
    /// Simple "Hello, world!" query
    fn hello_world() -> &str {
        "Hello, world!"
    }
    /// Client resource to query clients
    fn clients(ctx: &DataContext) -> Vec<&Client> {
        ctx.clients.iter().collect()
    }
}

pub type Mutation = EmptyMutation<DataContext>;

pub type Schema = RootNode<'static, Query, Mutation>;
