use sqlx::{Pool, Postgres, Row};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<Postgres>::connect(&database_url).await?;

    //creeate users table
//    create_table_users(&pool).await?;
    // Teste o CRUD
    create_user(&pool, "Alice", "alice@example.com").await?;
    create_user(&pool, "Bob", "bob@example.com").await?;

    let users = read_users(&pool).await?;
    println!("Users: {:?}", users);

    update_user(&pool, 1, "Alice Updated", "alice_updated@example.com").await?;
    
    let updated_user = read_user_by_id(&pool, 1).await?;
    println!("Updated User: {:?}", updated_user);

    delete_user(&pool, 2).await?;

    Ok(())
}


// create table
async fn create_table_users(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100),
            email VARCHAR(100) UNIQUE NOT NULL
        )
    "#)
    .execute(pool) // Remova o '&' antes de pool
    .await?;

    println!("Tabela 'users' criada com sucesso!");

    Ok(())
}


// CREATE
async fn create_user(pool: &Pool<Postgres>, name: &str, email: &str) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO users (name, email) VALUES ($1, $2)";
    sqlx::query(query)
        .bind(name)
        .bind(email)
        .execute(pool)
        .await?;
    println!("User created: {} - {}", name, email);
    Ok(())
}

// READ
async fn read_users(pool: &Pool<Postgres>) -> Result<Vec<(i32, String, String)>, sqlx::Error> {
    let query = "SELECT id, name, email FROM users";
    let rows = sqlx::query(query)
        .fetch_all(pool)
        .await?;

    let users: Vec<(i32, String, String)> = rows.into_iter()
        .map(|row| (row.get("id"), row.get("name"), row.get("email")))
        .collect();

    Ok(users)
}

async fn read_user_by_id(pool: &Pool<Postgres>, user_id: i32) -> Result<(i32, String, String), sqlx::Error> {
    let query = "SELECT id, name, email FROM users WHERE id = $1";
    let row = sqlx::query(query)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    let user = (row.get("id"), row.get("name"), row.get("email"));
    Ok(user)
}

// UPDATE
async fn update_user(pool: &Pool<Postgres>, user_id: i32, name: &str, email: &str) -> Result<(), sqlx::Error> {
    let query = "UPDATE users SET name = $1, email = $2 WHERE id = $3";
    sqlx::query(query)
        .bind(name)
        .bind(email)
        .bind(user_id)
        .execute(pool)
        .await?;
    println!("User updated: {} - {}", name, email);
    Ok(())
}

// DELETE
async fn delete_user(pool: &Pool<Postgres>, user_id: i32) -> Result<(), sqlx::Error> {
    let query = "DELETE FROM users WHERE id = $1";
    sqlx::query(query)
        .bind(user_id)
        .execute(pool)
        .await?;
    println!("User with id {} deleted", user_id);
    Ok(())
}

