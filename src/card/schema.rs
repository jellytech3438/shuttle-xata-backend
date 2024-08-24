use crate::*;

pub enum Image {
    Filename(String),
    File(String, Vec<u8>),
}

impl IntoResponse for Image {
    fn into_response(self) -> Response {
        match self {
            Self::Filename(name) => (axum::http::StatusCode::OK, name).into_response(),
            Self::File(filename, data) => {
                let filename_header_value = format!("attachment; filename=\"{filename}\"");

                Response::builder()
                    .header("Content-Disposition", filename_header_value)
                    .header("Content-Type", "image/jpeg")
                    .body(Body::from(data))
                    .unwrap()
            }
        }
    }
}

impl Into<Image> for (String, Vec<u8>) {
    fn into(self) -> Image {
        Image::File(self.0, self.1)
    }
}

impl Into<Image> for String {
    fn into(self) -> Image {
        Image::Filename(self)
    }
}

impl Into<Image> for &str {
    fn into(self) -> Image {
        Image::Filename(self.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct FormCard {
    pub id: String,
    pub uid: String,
    pub title: String,
    pub description: String,
    pub image: String,
}

impl FormCard {
    pub async fn insert_into_db(&self, pool: &PgPool) {
        let query_str = format!(
            "INSERT INTO \"card\" (user_id, title, description) VALUES (\'{}\', \'{}\', \'{}\')",
            &self.uid, &self.title, &self.description
        );
        sqlx::query(&query_str)
            .execute(pool)
            .await
            .expect("Failed to insert card into database");

        println!("Insert card into database");
    }

    pub async fn delete_from_db(&self, pool: &PgPool) {
        let query_str = format!("DELETE FROM \"card\" WHERE \"xata_id\" = \'{}\'", &self.id);
        sqlx::query(&query_str)
            .execute(pool)
            .await
            .expect("Failed to delete card from database");

        println!("Delete card from database");
    }

    pub async fn update(&self, pool: &PgPool, rhs: Self) {
        let query_str = format!("UPDATE \"card\" SET title = \'{}\', description = \'{}\', image = \'{}\' WHERE \"xata_id\" = \'{}\'", rhs.title, rhs.description, rhs.image, &self.id);
        sqlx::query(&query_str)
            .execute(pool)
            .await
            .expect("Failed to update card");

        println!("Update card");
    }

    pub fn test_data() -> Self {
        FormCard {
            id: "test".to_owned(),
            uid: "test".to_owned(),
            title: "test".to_owned(),
            description: "test".to_owned(),
            image: "test".to_owned(),
        }
    }
}
