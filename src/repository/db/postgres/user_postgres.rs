use crate::domain::{
    model::user_model::{Entity as UserEntity, Model as User},
    user_domain::UserRepository,
};
use async_trait::async_trait;
use sea_orm::{prelude::*, DatabaseConnection};
use std::sync::Arc;

pub struct UserRepo {
    pub conn: Arc<DatabaseConnection>,
}
impl UserRepo {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}
#[async_trait]
impl UserRepository for UserRepo {
    async fn get_all(&self) -> Vec<User> {
        UserEntity::find().all(&*self.conn).await.unwrap()
    }
    async fn get_by_id(&self, id: i32) -> Result<User, String> {
        Err("hoge".to_string())
    }
}
