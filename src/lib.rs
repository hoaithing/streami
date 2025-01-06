use crate::schema::api_sim::dsl::api_sim;
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use std::env;
use chrono::prelude::*;

pub mod models;
pub mod schema;

use self::models::*;
use crate::schema::api_simidmapper;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_sims(page: i64, page_size: i64) -> Result<Vec<Sim>, diesel::result::Error> {
    let conn = &mut establish_connection();
    let offset = (page - 1) * page_size;
    let res: Vec<Sim> = api_sim
        .offset(offset)
        .limit(page_size)
        .select(Sim::as_select())
        .load(conn)
        .expect("Error loading posts");
    Ok(res)
}

pub fn add_sim_mapper(
    imsi: String,
    iccid: &String,
    msisdn: String,
    qr_code: String,
    esim: bool,
    provider: &String,
) -> QueryResult<SimMapper> {
    let conn = &mut establish_connection();
    let new_sim = SimMapperInsert {
        imsi,
        iccid: iccid.clone(),
        esim,
        provider: provider.clone(),
        qr_code: Some(qr_code),
        synced: false,
        last_email: None,
        msisdn: Some(msisdn),
        active: false,
        booking_id: None,
        created: Some(Utc::now()),
        updated: Default::default(),
        assigned: false,
        joytel_pin: "".to_string(),
    };

    diesel::insert_into(api_simidmapper::table)
        .values(&new_sim)
        .returning(SimMapper::as_returning())
        .get_result(conn)

}
