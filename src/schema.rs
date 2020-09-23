use juniper::{self, EmptyMutation, RootNode};

pub struct Query;

#[juniper::object]
/// An example GraphQL Schema built with Rust
impl Query {
    /// Simple "Hello, world!" query
    fn hello_world() -> &str {
        "Hello, world!"
    }
}

pub type Mutation = EmptyMutation<()>;

pub type Schema = RootNode<'static, Query, Mutation>;
