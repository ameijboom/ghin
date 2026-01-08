use diesel::OptionalExtension;
use std::sync::{Arc, Mutex};

use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tokio::task;

use crate::store::models::{Config, NewConfig};

pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct Database {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            conn: Arc::new(Mutex::new(SqliteConnection::establish(path)?)),
        })
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        self.execute_blocking(move |mut conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .map_err(|e| anyhow::anyhow!(e))?;

            Ok(())
        })
        .await
    }

    pub async fn get_config(&self) -> anyhow::Result<Config> {
        self.execute_blocking(move |mut conn| {
            schema::config::table
                .first::<Config>(&mut *conn)
                .map_err(|e| anyhow::anyhow!(e))
        })
        .await
    }

    pub async fn set_config(&self, config: NewConfig) -> anyhow::Result<Option<Config>> {
        if config.installation_directory.is_none() && config.temp_directory.is_none() {
            return Ok(None);
        }

        self.execute_blocking(move |mut conn| {
            let current: Option<Config> = schema::config::table.first(&mut *conn).optional()?;

            if let Some(c) = &current
                && c.installation_directory.to_str() == config.installation_directory.as_deref()
                    && c.temp_directory.to_str() == config.temp_directory.as_deref()
                {
                    return Ok(None);
                }

            if current.is_some() {
                diesel::update(schema::config::table)
                    .set(&config)
                    .execute(&mut *conn)?;
            } else {
                diesel::insert_into(schema::config::table)
                    .values(&config)
                    .execute(&mut *conn)?;
            }

            Ok(current)
        })
        .await
    }

    async fn execute_blocking<F, T>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(std::sync::MutexGuard<SqliteConnection>) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let conn = Arc::clone(&self.conn);
        task::spawn_blocking(move || f(conn.lock().unwrap())).await?
    }
}
