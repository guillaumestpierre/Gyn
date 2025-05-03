#![allow(non_snake_case)]
use dioxus::prelude::*;
use chrono::NaiveDate;
use rusqlite::params;
use crate::db::get_db_connection;
use crate::{models::training::Exercise, routes::routes::Route::Home};
use crate::components::old_exos::OldExo;
use crate::models::r#const::{TABLE_EXERCISES, TABLE_REPS};


fn fetch_exercises() -> Result<Vec<Exercise>, rusqlite::Error>{
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();
    let mut res_exos: Vec<Exercise> = Vec::new();

    let mut stmt_exo = conn.prepare(&format!("SELECT exid, name, date, starter FROM {}", 
        TABLE_EXERCISES))?;

    let exo_rows = stmt_exo.query_map([], |row| {
        let exid: u32 = row.get(0)?;
        let name: String = row.get(1)?;
        let date_str: String = row.get(2)?;
        let is_starter: bool = row.get(3)?;
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                0, rusqlite::types::Type::Text, Box::new(e)))?;
        Ok((exid, name, date, is_starter))
    })?;

    for row in exo_rows{
        if let Ok((exid, name, date, starter)) = row{
            let mut stmt_reps = conn.prepare(&format!(
                "SELECT repnum, weight FROM {} WHERE exid = ?1", TABLE_REPS
            ))?;
            let rep_rows = stmt_reps.query_map([exid], |row|{
                Ok((row.get(0)?, row.get(1)?))
            })?;
            let reps: Vec<(u32, f32)> = rep_rows.collect::<Result<_, _>>()?;
            res_exos.push(Exercise{
                exid,
                name,
                reps,
                date,
                starter
            });
        }
    }
    Ok(res_exos)
}

fn save_data(exo: Exercise) -> rusqlite::Result<()> {
    let conn = get_db_connection();
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;

    {
        let query_exo = format!("UPDATE {} SET name = ?1, date = ?2, starter = ?3 WHERE exid = ?4", TABLE_EXERCISES);
        let mut stmt_exo = tx.prepare(&query_exo)?;
        stmt_exo.execute(params![
            exo.name,
            exo.date.to_string(),
            exo.starter,
            exo.exid
        ])?;    

        let query_clean_reps = format!("DELETE FROM {} WHERE exid = ?1", TABLE_REPS);
        let mut stmt_clean_reps = tx.prepare(&query_clean_reps)?;
        stmt_clean_reps.execute(params![exo.exid])?;

        let query_reps = format!("INSERT INTO {} (exid, repnum, weight) VALUES (?1, ?2, ?3)",TABLE_REPS);
        let mut stmt_reps = tx.prepare(&query_reps)?;
        for (num, weight) in exo.reps {
            stmt_reps.execute(params![
                exo.exid, 
                num, 
                weight, 
            ])?;
        }
    }
    tx.commit()?;

    Ok(())
}

fn delete_data(id:u32) -> rusqlite::Result<()> {
    let conn = get_db_connection();
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;

    {  

        let query_del_reps = format!("DELETE FROM {} WHERE exid = ?1", TABLE_REPS);
        let mut stmt_reps = tx.prepare(&query_del_reps)?;
        stmt_reps.execute(params![id])?;

        let query_del_exo = format!("DELETE FROM {} WHERE exid = ?1", TABLE_EXERCISES);
        let mut stmt_exo = tx.prepare(&query_del_exo)?;
        stmt_exo.execute(params![id])?;

    }
    tx.commit()?;

    Ok(())
}

pub fn Hist() -> Element {

    let data = fetch_exercises();
    let mut exercises = use_signal( || match data {
        Ok(_) => {data.unwrap()},
        Err(_)=>{Vec::new()}
    });

    let mut update_exercise = move |id: u32, updated_exercise: Exercise| {
        let mut current_exercises = exercises.read().clone();
        if let Some(index) = current_exercises.iter().position(|ex| ex.exid == id) {
            match save_data(updated_exercise.clone()){
                Ok(_) =>{println!("Données sauvegardées");},
                Err(e)=>{println!("Erreur lors de l'enregistrement: {}", e);}
            };
            current_exercises[index] = updated_exercise;
            exercises.set(current_exercises);
        } else {
            println!("Exercice avec ID {} non trouvé", id);
        }
    };

    let mut delete_exercise = move |id: u32| {
        
        let mut current_exercises = exercises.read().clone();
        if let Some(index) = current_exercises.iter().position(|ex| ex.exid == id) {
            match delete_data(id) {
                Ok(_)=>{println!("suppression de l'exo avec succès");},
                Err(e)=>{println!("Échec de la suppression: {}", e);}
            }
            current_exercises.remove(index);          
            exercises.set(current_exercises);
        } else {
            println!("Exercice avec ID {} non trouvé", id);
        }
    };

    rsx! {
        div {
            class: "flex h-screen bg-neutral-300",   
            div {
                class: "w-64 h-full py-5 px-5 bg-neutral-200",
                
                button {
                    class: "w-full rounded-lg py-2 px-4 bg-cyan-50 text-lg font-bold text-left transition-colors duration-100 hover:bg-cyan-100",
                    onclick: move |_| {
                        let _ = navigator().push(Home {});
                    },
                    "Home"
                }
            }
            div {
                class: "flex flex-1 flex-col p-4 items-center space-y-6 overflow-y-auto",
                        
                div {
                    class: "grid grid-cols-1 gap-6 w-full px-4",
                    key: "{exercises.read().len()}",
                    {exercises.iter().map(|ex_with_id| {
                        let id = ex_with_id.exid;
                        rsx! {
                            OldExo {
                                key: "{id}",
                                exercise: Some(ex_with_id.clone()),
                                on_change: move |updated_exercise| update_exercise(id, updated_exercise),
                                on_delete: move |_| delete_exercise(id)
                            }
                        }
                    })}
                }
            }
        }
    }
}