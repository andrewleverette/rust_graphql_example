use juniper::{self, Context, EmptyMutation, RootNode};

use crate::db::DataContext;
use crate::models::*;

impl Context for DataContext {}

pub struct Query;

#[juniper::object(Context = DataContext)]
impl Query {
    fn hello_world() -> &str {
        "Hello, world!"
    }

    fn clients(&self, ctx: &DataContext) -> Vec<&Client> {
        ctx.clients.values().collect()
    }
}

pub type Mutation = EmptyMutation<DataContext>;

pub type Schema = RootNode<'static, Query, Mutation>;
