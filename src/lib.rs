use std::env;
use diesel::{Connection, PgConnection};
use diesel::prelude::*;

pub mod models;
pub mod schema;

use self::schema::api_sim::dsl::*;
use self::models::*;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_sims_with_pagination(
    page: i64,
    page_size: i64,
) -> Result<Vec<Sim>, diesel::result::Error> {
    let conn = &mut establish_connection();
    let offset = (page - 1) * page_size;
    let res: Vec<Sim> = api_sim.offset(offset).limit(page_size).select(Sim::as_select()).load(conn).expect("Error loading posts");
    res
}
