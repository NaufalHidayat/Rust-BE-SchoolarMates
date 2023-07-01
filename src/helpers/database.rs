use std::env;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::postgres::{PgPool, PgPoolOptions};

lazy_static! {
  pub static ref POOL: AsyncOnce<PgPool> = AsyncOnce::new(async  {
    let client = open_postgres().await;

    client
  });
}

#[doc = "Open connection to postgres database"]
pub async fn open_postgres() -> PgPool {
  let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| String::from("postgres://postgres:postgres@localhost:5137/postgres"));
  
  println!("database connect to {}", database_url);

  let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    .expect("Can't connect to database");

  pool
}

#[doc = "Connect to postgres database"]
pub async fn connect_postgres() -> PgPool {
  let db = POOL.get().await.to_owned();

  db
}