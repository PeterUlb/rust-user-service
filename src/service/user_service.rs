use crate::model::User;
use crate::repository::user_repository::UserRepository;
use rand::Rng;

#[derive(Debug)]
pub enum UserServiceError {
    GenericDatabaseError(diesel::result::Error),
}

impl From<diesel::result::Error> for UserServiceError {
    fn from(error: diesel::result::Error) -> UserServiceError {
        UserServiceError::GenericDatabaseError(error)
    }
}

pub fn register_user(
    user_repository: &dyn UserRepository,
    username: &str,
    argon2_config: &argon2::Config,
) -> i32 {
    let salt: String = rand::rngs::OsRng
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .collect();
    let hash =
        argon2::hash_encoded("mypassword".as_bytes(), salt.as_bytes(), argon2_config).unwrap();
    info!("{}", hash);
    return if username == "Hans" {
        user_repository.get_user_by_id(333)
    } else {
        user_repository.get_user_by_id(111)
    };
}

pub fn get_all_user(user_repository: &dyn UserRepository) -> Result<Vec<User>, UserServiceError> {
    return user_repository.get_all().map_err(|e| e.into());
}

#[cfg(test)]
mod tests {

    use crate::model::User;
    use crate::repository::user_repository::UserRepository;
    use chrono::NaiveDate;
    use chrono::Utc;
    use diesel::QueryResult;

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
        let user_repo = MockUserRepo {};
        let users = match super::get_all_user(&user_repo) {
            Ok(users) => users,
            Err(e) => panic!(e),
        };
        assert_eq!(users.len(), 2);
    }
}
