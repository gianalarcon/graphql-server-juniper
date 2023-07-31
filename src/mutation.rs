use juniper::{graphql_object, FieldResult};

use crate::schema::{DatabaseContext, User, UserInput};

pub struct MutationRoot;

#[graphql_object(context = DatabaseContext)]
impl MutationRoot {
    fn create_user(context: &DatabaseContext, user: UserInput) -> FieldResult<User> {
        let mut write = context.0.write().expect("Can not access database");
        let user = User {
            id: user.id,
            name: user.name.clone(),
        };
        write.insert_user(user.clone());
        Ok(user)
    }
}
