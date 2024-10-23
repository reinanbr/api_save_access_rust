use sqlx::postgres::PgPoolOptions;
//use sqlx::PgPoolOptions;
use dotenv::dotenv;
use std::env;
//use models::access_site_model::create_access_site;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Conexão ao pool de conexão com SQLx
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Exemplo de criação de um novo registro
    models::access_sites_model::create_access_site(
        &pool, 
        "example.com", 
        "192.168.0.1", 
        "example-host", 
        "2024-10-17 10:30:00", 
        "Example ISP", 
        "New York", 
        "NY", 
        "USA"
    ).await?;

    Ok(())
}

