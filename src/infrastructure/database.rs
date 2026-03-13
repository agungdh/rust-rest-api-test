use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use std::path::PathBuf;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn get_db_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("data");
    std::fs::create_dir_all(&path).ok();
    path.push("employees.db");
    path
}

pub fn establish_connection() -> DbPool {
    let db_path = get_db_path();
    let db_url = db_path.to_string_lossy().to_string();

    let manager = ConnectionManager::<SqliteConnection>::new(&db_url);
    let pool = Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Failed to create pool for database at {:?}", db_path));

    pool
}
