use crate::models::Sim;
use crate::schema::api_sim;
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Debug, Clone)]
pub struct SimQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub provider: Option<String>,
}

impl SimQuery {
    fn apply_query<'a>(&'a self, query: api_sim::BoxedQuery<'a, Pg>) -> api_sim::BoxedQuery<'a, Pg> {
        let mut query = query;

        let page_size = self.page_size.unwrap_or(10);
        query = query
            .offset((self.page_size.unwrap_or(1) - 1) * page_size)
            .limit(page_size);

        if let Some(provider) = &self.provider {
            query = query.filter(api_sim::provider.eq(provider));
        }

        if let Some(search) = &self.search {
            let pattern = format!("%{}%", search);
            query = query.filter(
                api_sim::sim_id
                    .ilike(pattern.clone())
                    .or(api_sim::sim_serial.ilike(pattern.clone())),
            );
        }
        query
    }

    pub fn execute(&self, conn: &mut PgConnection) -> QueryResult<Vec<Sim>> {
        let query = api_sim::table.into_boxed::<Pg>();
        self.apply_query(query).load::<Sim>(conn)
    }
}

#[derive(Deserialize)]
pub struct FileContentQuery {
    pub file_name: String,
    pub line: Option<usize>,
    pub num_lines: Option<usize>,
    pub search: Option<String>,
}

#[derive(Serialize)]
pub struct FileContentResponse {
    pub content: Vec<String>,
}

#[derive(Serialize)]
pub struct SimResponse {
    pub count: i64,
    pub next: &'static str,
    pub prev: Option<&'static str>,
    pub results: Vec<Sim>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimMapper {
    pub id: i64,
    pub iccid: String,
    pub sim_number: String,
    pub sim_serial: String,
    pub qr_code: String,
    pub added: bool,
    pub provider: String,
    pub synced: bool,
    pub synced_at: Option<String>,
    pub product_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub type SearchResult = io::Result<(usize, Option<Vec<(usize, String)>>)>;
