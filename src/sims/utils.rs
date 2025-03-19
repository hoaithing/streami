use crate::sims::serializers::DynamicFilters;
use sqlx::postgres::PgRow;
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres, QueryBuilder, Row};
use std::env;
use tracing::warn;

pub async fn create_pool() -> Pool<Postgres> {
    let database_env = env::var("DATABASE_URL");
    if database_env.is_ok() {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_env.unwrap())
            .await
            .expect("Failed to create pool")
    } else {
        panic!("{:?}", database_env.unwrap());
    }
}

impl DynamicFilters {
    pub fn build_where_clause<'a>(
        &'a self,
        query_builder: &'a mut QueryBuilder<'a, Postgres>,
    ) -> &'a mut QueryBuilder<'a, Postgres> {
        query_builder.push(" WHERE 1=1");

        if let Some(search) = &self.search {
            if !search.is_empty() {
                if let Some(search_fields) = &self.search_fields {
                    for (index, field) in search_fields.iter().enumerate() {
                        if index == 0 {
                            query_builder.push(format!(" AND {} ILIKE ", field));
                        } else {
                            query_builder.push(format!(" OR {} ILIKE ", field));
                        }
                        query_builder.push_bind(format!("%{}%", search));
                    }
                }
            }
        }

        // Process all dynamic fields except pagination and sorting
        for (key, value) in self.fields.iter() {
            // Skip pagination and sorting parameters
            if value.is_empty() {
                continue;
            }
            if !["page", "page_size", "sort_by", "sort_order", "search"].contains(&key.as_str()) {
                // Handle different operators in the field name
                if key.contains("__") {
                    let parts: Vec<&str> = key.split("__").collect();
                    let field_name = parts[0];
                    let operator = parts[1];

                    match operator {
                        "gt" => {
                            query_builder.push(format!(" AND {} > ", field_name));
                            query_builder.push_bind(value);
                        }
                        "lt" => {
                            query_builder.push(format!(" AND {} < ", field_name));
                            query_builder.push_bind(value);
                        }
                        "gte" => {
                            query_builder.push(format!(" AND {} >= ", field_name));
                            query_builder.push_bind(value);
                        }
                        "lte" => {
                            query_builder.push(format!(" AND {} <= ", field_name));
                            query_builder.push_bind(value);
                        }
                        "in" => {
                            let values: Vec<&str> = value.split(',').collect();
                            if !values.is_empty() {
                                query_builder.push(format!(" AND {} IN (", field_name));
                                let mut separated = query_builder.separated(", ");
                                for value in values {
                                    separated.push_bind(value.trim().to_string());
                                }
                                separated.push_unseparated(")");
                            }
                        }
                        _ => {
                            query_builder.push(format!(" AND {} = ", field_name));
                            query_builder.push_bind(value);
                        }
                    }
                } else {
                    // Default to exact match if no operator is specified
                    query_builder.push(format!(" AND {} = ", key));
                    query_builder.push_bind(value);
                }
            }
        }
        if let Some(esim) = self.esim {
            query_builder.push(" AND esim = ");
            query_builder.push_bind(esim);
        }
        if let Some(active) = self.active {
            query_builder.push(" AND active = ");
            query_builder.push_bind(active);
        }
        query_builder
    }

    pub fn get_sort_clause(&self) -> String {
        let sort_by = self.sort_by.as_deref().unwrap_or("created");
        let sort_order = self.sort_order.as_deref().unwrap_or("DESC");

        format!(" ORDER BY {} {}", sort_by, sort_order)
    }

    pub fn get_pagination(&self) -> (i64, i64) {
        let page_size = self.page_size.unwrap_or(50);
        let page = self.page.unwrap_or(1);
        let offset = (page - 1) * page_size;

        (page_size, offset)
    }
}

pub async fn get_data_from_db<T>(
    pool: Pool<Postgres>,
    filters: DynamicFilters,
    table: &str,
    select_columns: Option<&str>,
) -> Result<(i64, Vec<T>), sqlx::Error>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let columns = select_columns.unwrap_or("*");
    let (limit, offset) = filters.get_pagination();
    let sort_clause = filters.get_sort_clause();

    // Start building the query
    let mut query_builder = QueryBuilder::<Postgres>::new(format!(
        r#"
        WITH data AS (
            SELECT {columns}
            FROM {table}
        "#,
        columns = columns,
        table = table,
    ));

    // Add WHERE clause and bindings
    let query_builder = filters.build_where_clause(&mut query_builder);

    // Add sorting
    query_builder.push(&sort_clause);

    // Add LIMIT and OFFSET
    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);
    query_builder.push(")");

    let column_list: Vec<&str> = columns.split(",").collect();

    let mut f = String::new();
    for (index, column) in column_list.clone().into_iter().enumerate() {
        f.push_str("data.");
        f.push_str(column);
        if index < column_list.len() - 1 {
            f.push(',');
        }
    }
    query_builder.push(format!(
        r#"
        SELECT
            (SELECT COUNT(*) FROM {table}) AS total_count, {f}
        FROM data;
        "#,
        table = table,
        f = f
    ));

    // Debug: Print the SQL query
    // warn!("Query SQL: {}", query_builder.sql());
    // Execute the query
    let rows = query_builder.build().fetch_all(&pool).await?;

    let mut results = Vec::new();
    let mut total_count = 0;

    // Process results
    if let Some(first_row) = rows.first() {
        total_count = first_row.try_get("total_count").unwrap_or(0);
    }

    for row in rows {
        let r = T::from_row(&row);
        match r {
            Ok(item) => {
                results.push(item);
            }
            Err(e) => {
                warn!("Deserializer Data error: {}", e);
            }
        }
    }
    Ok((total_count, results))
}
