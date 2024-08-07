use sqlx::{Pool, Postgres};

pub struct DbConn {
    pub db: Pool<Postgres>,
}
