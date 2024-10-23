use sqlx::PgPool;
use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct AccessSite {
    pub id: i32,
    pub site: String,
    pub ip: String,
    pub hostname: String,
    pub date: NaiveDateTime,
    pub provedor: String,
    pub city: String,
    pub state: String,
    pub country: String,
}

// Função para criar um novo registro de acesso ao site
pub async fn create_access_site(
    pool: &PgPool, 
    site: &str, 
    ip: &str, 
    hostname: &str, 
    date: &str, 
    provedor: &str, 
    city: &str, 
    state: &str, 
    country: &str
) -> Result<(), sqlx::Error> {
    // Parse da data usando chrono
    let date_time = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    // Converte o NaiveDateTime para String antes de bind
    let date_time_str = date_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Inserção no banco de dados
    sqlx::query("INSERT INTO table_access_sites (site, ip, hostname, date, provedor, city, state, country) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(site)
        .bind(ip)
        .bind(hostname)
        .bind(date_time_str)  // Convertemos o NaiveDateTime para String
        .bind(provedor)
        .bind(city)
        .bind(state)
        .bind(country)
        .execute(pool)
        .await?;

    println!("Novo registro criado: {}", site);
    Ok(())
}

