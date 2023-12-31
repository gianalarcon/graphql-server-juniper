use crate::{mutation::MutationRoot, query::QueryRoot};
use juniper::{EmptySubscription, GraphQLInputObject, GraphQLObject, RootNode};
use std::{collections::HashMap, sync::RwLock};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<DatabaseContext>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::<DatabaseContext>::default(),
    )
}

#[derive(GraphQLObject, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(GraphQLInputObject, Clone)]
pub struct UserInput {
    pub id: i32,
    pub name: String,
}

#[derive(Default)]
pub struct DatabaseContext(pub RwLock<Database>);
impl DatabaseContext {
    pub fn new() -> DatabaseContext {
        DatabaseContext(RwLock::new(Database::new()))
    }
}

#[derive(Default)]
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

    pub fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
}
