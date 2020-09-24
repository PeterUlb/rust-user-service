use crate::model::User;
use crate::repository::UserRepository;
use std::sync::Arc;

#[derive(Debug)]
pub enum ServiceError {
    GenericDatabaseError(diesel::result::Error),
}

impl From<diesel::result::Error> for ServiceError {
    fn from(error: diesel::result::Error) -> ServiceError {
        ServiceError::GenericDatabaseError(error)
    }
}

pub trait UserService: Send + Sync {
    fn register_user(&self, username: &str) -> i32;
    fn get_all_user(&self) -> Result<Vec<User>, ServiceError>;
}

pub struct UserServiceImpl {
    user_repository: Arc<Box<dyn UserRepository>>,
}
impl UserServiceImpl {
    pub fn new(user_repository: Arc<Box<dyn UserRepository>>) -> UserServiceImpl {
        println!("New user service");
        return UserServiceImpl { user_repository };
    }
}
impl UserService for UserServiceImpl {
    fn register_user(&self, username: &str) -> i32 {
        return if username == "Hans" {
            self.user_repository.get_user_by_id(333)
        } else {
            self.user_repository.get_user_by_id(111)
        };
    }

    fn get_all_user(&self) -> Result<Vec<User>, ServiceError> {
        return self.user_repository.get_all().map_err(|e| e.into());
    }
}
