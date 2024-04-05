use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FormUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub phone_number: String,
}

impl FormUser {
    pub async fn insert_into_db(&self, pool: &MySqlPool) {
        sqlx::query(r#"INSERT INTO `user` (id, name, email, phone_number) VALUES (?, ?, ?, ?)"#)
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.email)
            .bind(&self.phone_number)
            .execute(pool)
            .await
            .expect("Failed to insert user into database");

        println!("Insert user into database");
    }
}
