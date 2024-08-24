use crate::*;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct FormUser {
    pub id: String,
    pub name: String,
    pub mail: String,
    pub password: String,
}

impl FormUser {
    pub async fn insert_into_db(&self, pool: &PgPool) {
        let query_str = format!(
            "INSERT INTO \"user\" (name, mail, password) VALUES (\'{}\', \'{}\', \'{}\')",
            &self.name, &self.mail, &self.password
        );
        sqlx::query(&query_str)
            .execute(pool)
            .await
            .expect("Failed to insert user into database");

        println!("Insert user into database");
    }

    pub async fn delete_from_db(&self, pool: &PgPool) {
        let query_str = format!("DELETE FROM \"user\" WHERE \"xata_id\" = \'{}\'", &self.id);
        sqlx::query(&query_str)
            .execute(pool)
            .await
            .expect("Failed to delete user from database");

        println!("Delete user from database");
    }

    pub async fn update(&self, pool: &PgPool, rhs: Self) {
        let query_str = format!("UPDATE \"user\" SET name = \'{}\', mail = \'{}\', password = \'{}\' WHERE \"xata_id\" = \'{}\'", rhs.name, rhs.mail, rhs.password, &self.id);
        sqlx::query(&query_str)
            .execute(pool)
            .await
            .expect("Failed to update user");

        println!("Update user");
    }

    pub fn test_data() -> Self {
        FormUser {
            id: "test".to_owned(),
            name: "test".to_owned(),
            mail: "test".to_owned(),
            password: "test".to_owned(),
        }
    }
}
