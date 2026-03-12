use rusqlite::Connection;
use std::path::PathBuf;

pub fn get_db_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("data");
    std::fs::create_dir_all(&path).ok();
    path.push("app.db");
    path
}

pub fn establish_connection() -> Connection {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path)
        .expect(&format!("Failed to connect to database at {:?}", db_path));

    run_migrations(&conn);

    conn
}

pub fn run_migrations(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS departments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT
        );
        
        CREATE TABLE IF NOT EXISTS employees (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            position TEXT NOT NULL,
            salary INTEGER NOT NULL,
            department_id INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT,
            FOREIGN KEY (department_id) REFERENCES departments(id)
        );
        ",
    )
    .expect("Failed to run migrations");
}
