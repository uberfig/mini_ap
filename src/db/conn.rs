use deadpool_postgres::Pool;

use crate::activitystream_objects::actors::Actor;

pub struct DbConn {
    pub db: Pool,
}

pub enum PermissionLevel {
    /// intended for the main admin account(s) of the server, will be featured
    /// can be set to be auto followed by new users
    AdminOne, 
    /// intended for anyone who has admin access to the server
    AdminTwo, 
    /// intended for mods who can take vito actoin in an emergency
    ModOne,
    /// intended for mods who need to open a proposal for mod changes
    ModTwo,
    /// intended for public registration servers to limit things to only
    /// known users for example if they wish to have only known users or
    /// higher able to vote on proposals so that a malicious actor can't
    /// start making accounts to influence a decision. When manual approval
    /// is used, all approved users will be trusted and pending users will
    /// be untrusted. this would allow for a switching to manual approval
    /// in the event of an emergency still allowing trusted users to 
    /// continue unnaffected and untrusted accounts would be preserved and
    /// prompted to send an application for approval when they log in next
    TrustedUser,
    /// the default, what they can do is up to server policy, used for 
    /// accounts pending approval in a manual approval setup
    UntrustedUser,
}
impl From<u16> for PermissionLevel {
    fn from(value: u16) -> Self {
        match value {
            1 => PermissionLevel::AdminOne,
            2 => PermissionLevel::AdminTwo,
            3 => PermissionLevel::ModOne,
            4 => PermissionLevel::ModTwo,
            5 => PermissionLevel::TrustedUser,
            6 => PermissionLevel::UntrustedUser,
            _ => PermissionLevel::UntrustedUser,
        }
    }
}
impl From<PermissionLevel> for u16 {
    fn from(val: PermissionLevel) -> Self {
        match val {
            PermissionLevel::AdminOne => 1,
            PermissionLevel::AdminTwo => 2,
            PermissionLevel::ModOne => 3,
            PermissionLevel::ModTwo => 4,
            PermissionLevel::TrustedUser => 5,
            PermissionLevel::UntrustedUser => 6,
        }
    }
}

pub trait Conn {
    async fn get_private_key(&self, preferred_username: &str) -> String;
    // async fn get_public_key(&self, owner: &str) -> String;
    // async fn insert_public_key(
    //     &self,
    //     id: &str,
    //     actor_id: &str,
    //     public_key_pem: &str,
    // ) -> Result<i64, ()>;
    
    async fn create_federated_user(&self, actor: Actor) -> i64;
    async fn get_federated_user_db_id(&self, actor_id: &str) -> Option<i64>;
    async fn get_federated_actor(&self, actor_id: &str) -> Option<Actor>;
    async fn get_federated_actor_db_id(&self, id: i64) -> Option<Actor>;
    
    async fn create_local_user(
        &self,
        username: String,
        password: String,
        instance_domain: &str,
    ) -> Result<i64, ()>;
    async fn get_local_user_db_id(&self, preferred_username: &str) -> Option<i64>;
    async fn get_local_user_actor(&self, preferred_username: &str) -> Option<Actor>;
    async fn get_local_user_actor_db_id(&self, id: i64) -> Option<Actor>;
}
