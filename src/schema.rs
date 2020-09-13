use juniper::{self, EmptyMutation, RootNode};

pub struct Query;

#[juniper::object]
impl Query {
    fn hello_world() -> &str {
        "Hello, world!"
    }
}

pub type Mutation = EmptyMutation<()>;

pub type Schema = RootNode<'static, Query, Mutation>;
