use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env var must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    return Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");
}
