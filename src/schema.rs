use juniper::{self, EmptyMutation, RootNode};

use crate::models::*;

pub struct Query;

#[juniper::object]
/// An example GraphQL Schema built with Rust
impl Query {
    /// Simple "Hello, world!" query
    fn hello_world() -> &str {
        "Hello, world!"
    }

     /// Client resource to query clients
    fn client() -> Client {
        Client {
            client_id: "07-5583691".to_owned(),
            company_name: "Rutherford, Buckridge and Gibson".to_owned(),
            contact_name: "Xylia Froome".to_owned(),
            contact_title: "Legal Assistant".to_owned(),
            phone: "931-520-7757".to_owned(),
            email: "xfroome0@mac.com".to_owned(),
        }
    }
}

pub type Mutation = EmptyMutation<()>;

pub type Schema = RootNode<'static, Query, Mutation>;
