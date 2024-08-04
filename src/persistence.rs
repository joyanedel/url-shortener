use sqlx::{Pool, Postgres};

#[derive(sqlx::FromRow)]
pub struct UrlEntry {
    pub short_url: String,
    pub long_url: String,
}

pub async fn get_url_entry(connection: &Pool<Postgres>, short_url: &str) -> Option<UrlEntry> {
    sqlx::query_as::<_, UrlEntry>("SELECT short_url, long_url FROM urls WHERE short_url = ?")
        .bind(short_url)
        .fetch_one(connection)
        .await
        .ok()
}

pub async fn store_url_entry(connection: &Pool<Postgres>, url_entry: UrlEntry) -> Result<(), ()> {
    sqlx::query("INSERT INTO urls (short_url, long_url) VALUES ($1, $2)")
        .bind(url_entry.short_url)
        .bind(url_entry.long_url)
        .execute(connection)
        .await;

    Ok(())
}
