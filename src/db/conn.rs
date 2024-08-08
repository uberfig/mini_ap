use deadpool_postgres::Pool;

pub struct DbConn {
    pub db: Pool,
}
