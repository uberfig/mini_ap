use actix_web::web::Data;

use super::conn::Conn;

pub async fn process_inbox(conn: Data<Box<dyn Conn>>, state: Data<crate::config::Config>) {}
