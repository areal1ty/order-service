use sqlx::{PgPool, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // TODO: !!!
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/database")
        .await?;

    sqlx::query!("DELETE FROM items").execute(&pool).await?;
    sqlx::query!("DELETE FROM payment").execute(&pool).await?;
    sqlx::query!("DELETE FROM delivery").execute(&pool).await?;
    sqlx::query!("DELETE FROM orders").execute(&pool).await?;

    println!("All data has been cleared.");
    Ok(())
}
