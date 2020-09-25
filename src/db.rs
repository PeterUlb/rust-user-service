use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::DatabaseErrorKind;
use diesel::{r2d2, PgConnection};

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, diesel::result::Error> {
    pool.get().map_err(|e| {
        error!("{:?}", e);
        diesel::result::Error::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new("Could not get connection".to_owned()),
        )
    })
}
