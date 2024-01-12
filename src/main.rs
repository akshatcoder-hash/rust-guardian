use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::de::Unexpected::Str;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<RwLock<HashMap<String, User>>>;

#[derive(Clone)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[tokio::main]
async fn main() {
    let users = Arc::new(RwLock::new(init_users()));
}

fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User{
            uid: String::from("1"),
            email: String::from("akshatsharma0023@outlook.com"),
            pw: String::from("1234"),
            role:String::from("Admin"),
        },
    );
    map.insert(
        String::from("2"),
        User{
            uid: String::from("2"),
            email: String::from("akshatsharma.sol@outlook.com"),
            pw: String::from("1234"),
            role: String::from("User"),
        },
    );
    map
}