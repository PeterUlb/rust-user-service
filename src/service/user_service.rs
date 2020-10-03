use crate::model::users::{PasswordVersion, RegisterUserDto, UserStatus};
use crate::repository::user_repository::UserRepository;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum UserServiceError {
    DatabaseEntryAlreadyExists,
    GenericDatabaseError(diesel::result::Error),
}

impl From<diesel::result::Error> for UserServiceError {
    fn from(error: diesel::result::Error) -> UserServiceError {
        match error {
            diesel::result::Error::DatabaseError(db_error, _) => match db_error {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    UserServiceError::DatabaseEntryAlreadyExists
                }
                _ => UserServiceError::GenericDatabaseError(error),
            },
            _ => UserServiceError::GenericDatabaseError(error),
        }
    }
}

pub fn register_user(
    user_repository: &impl UserRepository, //equal to register_user<R> where R: UserRepository
    user_dto: RegisterUserDto,
    argon2_config: &argon2::Config,
) -> Result<usize, UserServiceError> {
    let mut user_dto = user_dto;
    let salt: String = rand::rngs::OsRng
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .collect();
    let hash =
        argon2::hash_encoded(user_dto.password.as_bytes(), salt.as_bytes(), argon2_config).unwrap();
    info!("{}", hash);
    user_dto.password = hash;

    let new_user = user_dto.into_new_user(PasswordVersion::ARGON2_1, UserStatus::NotVerified);
    user_repository.create_user(&new_user).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use crate::model::users::{NewUser, RegisterUserDto, User};
    use crate::repository::user_repository::UserRepository;
    use chrono::NaiveDate;
    use chrono::Utc;
    use diesel::QueryResult;

    struct MockUserRepo {
        scenario: i32,
    }

    struct MockErrorInfo {}
    impl diesel::result::DatabaseErrorInformation for MockErrorInfo {
        fn message(&self) -> &str {
            ""
        }

        fn details(&self) -> Option<&str> {
            None
        }

        fn hint(&self) -> Option<&str> {
            None
        }

        fn table_name(&self) -> Option<&str> {
            None
        }

        fn column_name(&self) -> Option<&str> {
            None
        }

        fn constraint_name(&self) -> Option<&str> {
            None
        }
    }

    impl UserRepository for MockUserRepo {
        fn create_user(&self, _: &NewUser) -> QueryResult<usize> {
            match self.scenario {
                1 => Ok(1),
                2 => Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    Box::new(MockErrorInfo {}),
                )),
                _ => Ok(1),
            }
        }

        fn get_user_by_username(&self, username: &str) -> QueryResult<Option<User>> {
            Ok(Some(User {
                id: 2,
                username: username.to_owned(),
                email: String::from("user2@example.com"),
                password: String::from("somepwhash"),
                password_version: 1,
                date_of_birth: NaiveDate::from_ymd(1992, 1, 1),
                status: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }))
        }
    }

    #[test]
    fn register_user() {
        let user_repo = MockUserRepo { scenario: 1 };
        let user_dto = RegisterUserDto {
            username: "MyUsername".to_owned(),
            email: "mail@mail.com".to_owned(),
            password: "somepassword".to_owned(),
            date_of_birth: NaiveDate::from_ymd(1990, 1, 1),
        };
        let result = super::register_user(&user_repo, user_dto, &argon2::Config::default());
        let expected: Result<usize, super::UserServiceError> = Ok(1);
        assert_eq!(expected, result);
    }

    #[test]
    fn register_user_exists() {
        let user_repo = MockUserRepo { scenario: 2 };
        let user_dto = RegisterUserDto {
            username: "MyUsername".to_owned(),
            email: "mail@mail.com".to_owned(),
            password: "somepassword".to_owned(),
            date_of_birth: NaiveDate::from_ymd(1990, 1, 1),
        };
        let result = super::register_user(&user_repo, user_dto, &argon2::Config::default());
        let expected: Result<usize, super::UserServiceError> =
            Err(super::UserServiceError::DatabaseEntryAlreadyExists);
        assert_eq!(expected, result);
    }
}
