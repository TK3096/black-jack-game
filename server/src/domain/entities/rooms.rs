use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct RoomEntity {
    pub id: i32,
    pub name: String,
    pub capacity: usize,
    pub owner_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
