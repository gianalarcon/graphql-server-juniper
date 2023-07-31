use crate::query::QueryRoot;
use juniper::{EmptyMutation, EmptySubscription, GraphQLObject, RootNode};
use std::{collections::HashMap, hash::Hash};

pub type Schema =
    RootNode<'static, QueryRoot, EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot,
        EmptyMutation::<Database>::default(),
        EmptySubscription::<Database>::default(),
    )
}

#[derive(GraphQLObject, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub struct Database {
    users: HashMap<i32, User>,
}

impl Database {
    pub fn new() -> Database {
        let mut users = HashMap::new();
        users.insert(
            0,
            User {
                id: 1,
                name: "Alice".to_string(),
            },
        );
        users.insert(
            1,
            User {
                id: 2,
                name: "Bob".to_string(),
            },
        );
        Database { users }
    }

    pub fn get_user_by_id(&self, id: i32) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        Vec::from_iter(self.users.values())
    }
}
