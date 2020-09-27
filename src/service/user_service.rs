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

pub struct UserServiceImpl<R: UserRepository> {
    user_repository: Arc<R>,
}
impl<R: UserRepository> UserServiceImpl<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        info!("Created User Service");
        Self { user_repository }
    }
}
impl<R: UserRepository> UserService for UserServiceImpl<R> {
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

#[cfg(test)]
mod tests {
    use crate::model::User;
    use crate::repository::user_repository::UserRepository;
    use crate::service::user_service::UserService;
    use crate::service::user_service::UserServiceImpl;
    use chrono::NaiveDate;
    use chrono::Utc;
    use diesel::QueryResult;
    use std::sync::Arc;

    struct MockUserRepo {}

    impl UserRepository for MockUserRepo {
        fn get_all(&self) -> QueryResult<Vec<User>> {
            let users = vec![
                User {
                    id: 1,
                    username: String::from("User1"),
                    email: String::from("user1@example.com"),
                    password: String::from("somepwhash"),
                    password_version: 1,
                    date_of_birth: NaiveDate::from_ymd(1990, 1, 1),
                    status: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                User {
                    id: 2,
                    username: String::from("User2"),
                    email: String::from("user2@example.com"),
                    password: String::from("somepwhash"),
                    password_version: 1,
                    date_of_birth: NaiveDate::from_ymd(1992, 1, 1),
                    status: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            ];
            Ok(users)
        }
        fn get_user_by_id(&self, _id: i32) -> i32 {
            42
        }
    }

    #[test]
    fn get_all_user() {
        let user_repo = Arc::new(MockUserRepo {});
        let user_service = UserServiceImpl::new(user_repo);

        let users = match user_service.get_all_user() {
            Ok(users) => users,
            Err(e) => panic!(e),
        };
        assert_eq!(users.len(), 2);
    }
}
