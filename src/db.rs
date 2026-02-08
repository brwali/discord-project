use rusqlite::Connection;
use std::error::Error;
use rusqlite::params;

pub type DbPool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

/// Initialize DB file (apply schema) and return an r2d2 pool.
pub fn init_pool(db_path: &str) -> Result<DbPool, Box<dyn Error>> {
    // Ensure the DB file has the schema applied (use a single Connection for setup)
    let conn = Connection::open(db_path)?;
    let schema = include_str!("../schema.sql");
    conn.execute_batch(schema)?;

    // Create an r2d2 pool backed by sqlite
    let manager = r2d2_sqlite::SqliteConnectionManager::file(db_path);
    let pool = r2d2::Pool::builder().max_size(15).build(manager)?;

    Ok(pool)
}

pub fn add_generic_user(db: DbPool, discord_id: String) -> rusqlite::Result<()> {
    let conn = db.get().unwrap(); // r2d2 pooled connection
    match conn.execute(
        "INSERT INTO users (discord_id) VALUES (?1)",
        params![discord_id],
    ) {
        Ok(_size) => {
            Ok(())
        },
        Err(e) => {
            Err(e)
        }
    }
}