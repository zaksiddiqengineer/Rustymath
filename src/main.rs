use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use thiserror::Error;
use tracing::{error, info};

#[derive(Error, Debug)]
enum MyAppError {
    #[error("Environment variable not set: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Database connection error: {0}")]
    DatabaseConnectionError(#[from] sqlx::Error),
}

#[tokio::main]
async fn main() -> Result<(), MyAppError> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables from the .env file
    dotenv().ok();

    // Access the DATABASE_URL environment variable
    let database_url = get_database_url()?;

    // Log the database url for verification
    info!("Connection to the database at url :{}", database_url);

    // Create a connection pool to the SQLite database
    let pool = create_connection_pool(&database_url).await?;

    info!("Connected to the SQLite database");

    // Pass the pool to another function to interact with the database
    run_application(pool).await?;

    Ok(())
}

fn get_database_url() -> Result<String, MyAppError> {
    env::var("DATABASE_URL").map_err(MyAppError::from)
}

async fn create_connection_pool(database_url: &str) -> Result<SqlitePool, MyAppError> {
    SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .map_err(MyAppError::from)
}

async fn run_application(pool: SqlitePool) -> Result<(), MyAppError> {
    // Your application logic here, using the `pool`
    // This could include further function calls to handle specific tasks
    Ok(())
}
