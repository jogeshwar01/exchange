use sqlx::postgres::PgPoolOptions;

pub struct PostgresDb {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresDb {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await?;

        println!("Connected to Postgres - {}", db_url);

        Ok(Self { pool })
    }

    pub fn get_pg_connection(&self) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
        Ok(self.pool.clone())
    }
}
