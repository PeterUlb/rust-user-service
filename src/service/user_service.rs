use crate::model::User;
use crate::repository::user_repository::UserRepository;
use std::sync::Arc;

#[derive(Debug)]
pub enum UserServiceError {
    GenericDatabaseError(diesel::result::Error),
}

impl From<diesel::result::Error> for UserServiceError {
    fn from(error: diesel::result::Error) -> UserServiceError {
        UserServiceError::GenericDatabaseError(error)
    }
}

pub trait UserService: Send + Sync {
    fn register_user(&self, username: &str) -> i32;
    fn get_all_user(&self) -> Result<Vec<User>, UserServiceError>;
}

pub struct UserServiceImpl {
    user_repository: Arc<Box<dyn UserRepository>>,
}
impl UserServiceImpl {
    pub fn new(user_repository: Arc<Box<dyn UserRepository>>) -> Box<dyn UserService> {
        info!("Created User Service");
        return Box::new(UserServiceImpl { user_repository });
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

    fn get_all_user(&self) -> Result<Vec<User>, UserServiceError> {
        return self.user_repository.get_all().map_err(|e| e.into());
    }
}
