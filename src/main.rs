use jiff::Timestamp;
use jiff_sqlx::ToSqlx;
use sqlx::{postgres::PgPoolOptions, prelude::*, PgPool};

#[derive(FromRow)]
struct DbRow {
    id: String,
    name: String,
    created_at: jiff_sqlx::Timestamp,
}

#[derive(Debug)]
struct AppObject {
    id: String,
    name: String,
    created_at: jiff::Timestamp,
}

async fn save(input: &AppObject, db: PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO groups (id, name, created_at) VALUES ($1, $2, $3)", 
        input.id, 
        input.name, 
        input.created_at.to_sqlx())
        .execute(&db)
        .await
}

async fn load(db: PgPool) -> Result<Vec<AppObject>, sqlx::Error> {
    let result = sqlx::query_as!(DbRow, "SELECT id, name, created_at FROM groups")
        .fetch_all(&db)
        .await?
        .iter()
        .map(|db_row| AppObject {
            id: db_row.id,
            name: db_row.name,
            created_at: db_row.created_at.to_jiff()
        })
        .collect();
    Ok(result)
}

#[tokio::main]
async fn main() {
    let db = PgPoolOptions::new().connect("postgres://").await.unwrap();
    save(AppObject {
        id: "XYZ".into(),
        name: "ABC".into(),
        created_at: Timestamp::now()
    }, &db).await.unwrap();

    let fetched = load(db).await.unwrap();
    dbg!(fetched)
}
