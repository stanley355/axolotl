use sea_orm::{Database, DatabaseConnection};
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db_connection: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        AppState {
            db_connection: Self::connect_db().await,
        }
    }

    async fn connect_db() -> DatabaseConnection {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = Database::connect(db_url)
            .await
            .expect("Database connection failed");

        return connection;
    }
}
