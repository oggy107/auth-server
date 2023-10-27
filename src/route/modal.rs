use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Clone, Serialize)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pwd: String,
    pub role: Role,
}
