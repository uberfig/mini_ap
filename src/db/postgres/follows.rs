use crate::db::UserRef;

use super::pg_conn::PgConn;

pub async fn create_follow_request(conn: &PgConn, from_id: UserRef, to_id: UserRef) -> Result<(), ()> {
    todo!()
}

pub async fn approve_follow_request(conn: &PgConn, from_id: UserRef, to_id: UserRef) -> Result<(), ()> {
    todo!()
}

pub async fn get_followers(conn: &PgConn, preferred_username: UserRef) -> Result<(), ()> {
    todo!()
}

pub async fn get_follower_count(conn: &PgConn, preferred_username: UserRef) -> Result<(), ()> {
    todo!()
}
