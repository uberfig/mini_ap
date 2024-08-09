use deadpool_postgres::Pool;

pub struct PgConn {
    pub db: Pool,
}
