use crate::*;

#[derive(Serialize, Deserialize)]
pub struct LoginDetails {
    pub mail: String,
    pub password: String,
}

impl LoginDetails {
    pub async fn insert_into_db(&self, pool: &PgPool) {
        let query_str = format!(
            "INSERT INTO \"user\" (mail, password) VALUES (\'{}\', \'{}\')",
            &self.mail, &self.password
        );
        sqlx::query(&query_str)
            .execute(pool)
            .await
            .expect("Failed to insert user into database");

        println!("Insert user into database");
    }

    // pub async fn delete_from_db(&self, pool: &PgPool) {
    //     let query_str = format!("DELETE FROM \"user\" WHERE \"xata_id\" = \'{}\'", &self.id);
    //     sqlx::query(&query_str)
    //         .execute(pool)
    //         .await
    //         .expect("Failed to delete user from database");
    //
    //     println!("Delete user from database");
    // }
    //
    // pub async fn update(&self, pool: &PgPool, rhs: Self) {
    //     let query_str = format!(
    //         "UPDATE \"user\" SET mail = \'{}\', password = \'{}\' WHERE \"xata_id\" = \'{}\'",
    //         rhs.mail, rhs.password, &self.id
    //     );
    //     sqlx::query(&query_str)
    //         .execute(pool)
    //         .await
    //         .expect("Failed to update user");
    //
    //     println!("Update user");
    // }

    pub fn test_data() -> Self {
        LoginDetails {
            mail: "test".to_owned(),
            password: "test".to_owned(),
        }
    }
}
