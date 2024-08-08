use deadpool_postgres::Pool;

pub struct DbConn {
    pub db: Pool,
}

impl DbConn {
    pub async fn get_private_key(&self, preferred_username: &str) {
        
    }
}
