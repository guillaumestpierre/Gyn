use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use rand::Rng;
use chrono::{NaiveDate, Utc};

// Connexion globale à la base de données persistante
pub static DB_CONNECTION: Lazy<Arc<Mutex<Connection>>> = Lazy::new(|| {
    let app_data_dir = dirs::data_dir()
        .expect("Impossible de déterminer le dossier de données de l'application")
        .join("gyn-app");
    
    // Créer le répertoire s'il n'existe pas
    std::fs::create_dir_all(&app_data_dir).expect("Impossible de créer le répertoire de données");
    
    let db_path = app_data_dir.join("gyn.db");
    
    let conn = Connection::open(&db_path)
        .expect(&format!("Impossible d'ouvrir la base de données à {}", db_path.display()));
    
    Arc::new(Mutex::new(conn))
});

// Fonction pour obtenir la connexion
pub fn get_db_connection() -> Arc<Mutex<Connection>> {
    DB_CONNECTION.clone()
}

pub fn del_poids() -> Result<()> {
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();

    conn.execute("DELETE FROM weight", [])?;
    Ok(())
}

// Fonction pour initialiser les tables
pub fn create_tables() -> Result<()> {
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS exercise (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            reps   TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS training (
            id    INTEGER PRIMARY KEY,
            date  DATE,
            duration   INTEGER,
            exercices   TEXT NOT NULL,
            note   TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS weight (
            id    INTEGER PRIMARY KEY,
            date  DATE,
            weight INT
        )",
        [],
    )?;

    conn.execute(
        "DELETE FROM training",
        [],
    )?;

    /*
    // Place holder
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let date: chrono::NaiveDate = Utc::now().naive_utc().into();
        let duration: i32 = rng.gen_range(30..120);
        let exercices: String = format!("Exercice {}", rng.gen_range(1..5));
        let note: String = format!("Note {}", "rng.gen_range(1..10)bahbahbahbahbahbahbahabhabhbahbaabhabahbaha");

        conn.execute(
            "INSERT INTO training (date, duration, exercices, note) VALUES (?1, ?2, ?3, ?4)",
            &[&date.to_string(), &duration.to_string(), &exercices, &note],
        )?;
    }
    */

    Ok(())
}