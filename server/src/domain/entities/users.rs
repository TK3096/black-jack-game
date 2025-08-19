use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
