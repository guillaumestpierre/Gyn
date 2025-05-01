use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

pub static DB_CONNECTION: Lazy<Arc<Mutex<Connection>>> = Lazy::new(|| {
    let app_data_dir = dirs::data_dir()
        .expect("Impossible de déterminer le dossier de données de l'application")
        .join("gyn-app");
    
    std::fs::create_dir_all(&app_data_dir).expect("Impossible de créer le répertoire de données");
    
    let db_path = app_data_dir.join("gyn.db");
    
    let conn = Connection::open(&db_path)
        .expect(&format!("Impossible d'ouvrir la base de données à {}", db_path.display()));
    
    Arc::new(Mutex::new(conn))
});

pub fn get_db_connection() -> Arc<Mutex<Connection>> {
    DB_CONNECTION.clone()
}

pub fn create_tables() -> Result<()> {
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS exercises (
            id    INTEGER PRIMARY KEY,
            exid  INTEGER UNIQUE, 
            name  TEXT NOT NULL,
            date    DATE,
            starter BOOL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reps (
            id    INTEGER PRIMARY KEY,
            exid    INTEGER,
            repnum     INTEGER NOT NULL,
            weight  FLOAT,
            FOREIGN KEY (exid) REFERENCES exercises(exid)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS weight (
            id    INTEGER PRIMARY KEY,
            date  DATE,
            weight FLOAT
        )",
        [],
    )?;

    Ok(())
}