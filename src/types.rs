use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DieselError = diesel::result::Error;
