use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:server.db").await?;
    sqlx::query!(
        r#"
        INSERT INTO server (other)
        VALUES (31)
        "#
    )
    .execute(&pool)
    .await?;
    Ok(())
}
