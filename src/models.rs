use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_pg_mapper::{self, FromTokioPostgresRow};
use tokio_postgres::Row;

// #[derive(Deserialize, PostgresMapper, Serialize)]
#[derive(Deserialize, Serialize)]
// #[pg_mapper(table = "foo")]

// #[pg_mapper(table = "users")] // singular 'user' is a keyword..
pub struct User {
    pub email: String,
    pub username: String,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            email: row.get("email"),
            username: row.get("username"),
        }
    }
}

impl FromTokioPostgresRow for User {
    fn from_row(row: Row) -> Result<Self, tokio_pg_mapper::Error> {
        Ok(Self {
            email: row.get("email"),
            username: row.get("username"),
        })
    }

    fn from_row_ref(row: &Row) -> Result<Self, tokio_pg_mapper::Error> {
        Ok(Self {
            email: row.get("email"),
            username: row.get("username"),
        })
    }

    fn sql_table() -> String {
        "User".to_string()
    }

    fn sql_fields() -> String {
        todo!()
    }

    fn sql_table_fields() -> String {
        todo!()
    }
}