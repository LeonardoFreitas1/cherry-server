type SqlDateTime = chrono::NativeDateTime;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub status: String,
    pub created_at: SqlDateTime,
    pub updated_at: SqlDateTime,
}

