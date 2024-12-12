use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:client.db").await?;
    sqlx::query!(
        r#"
        INSERT INTO client (thing)
        VALUES (31)
        "#
    )
    .execute(&pool)
    .await?;
    Ok(())
}
