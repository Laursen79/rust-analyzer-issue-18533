use std::error::Error;

#[tokio::main]
async fn main() -> Box<dyn Error> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:client.db").await?;
    sqlx::query!(
        r#"
            SELECT thing FROM Client
            WHERE id = 21
            "#
    )
    .execute(&pool)
    .await?
}
