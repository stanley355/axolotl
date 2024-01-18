use sea_orm::{Database, DatabaseConnection};
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db_conection: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        AppState {
            db_conection: Self::connect_db().await,
        }
    }

    async fn connect_db() -> DatabaseConnection {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Database::connect(db_url)
            .await
            .expect("Database connection failed")
    }
}
