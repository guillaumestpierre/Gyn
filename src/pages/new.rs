#![allow(non_snake_case)]
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use std::vec;
use chrono::NaiveDate;
use dioxus::prelude::*;
use rusqlite::params;
use crate::models::r#const::{TABLE_EXERCISES, TABLE_REPS};
use crate::components::new_exos::NewExo;
use crate::models::training::Exercise;
use crate::routes::routes::Route::Home;
use crate::db::get_db_connection;

fn save_data(exercises: Vec<Exercise>, date: NaiveDate) -> rusqlite::Result<()> {
    let conn = get_db_connection();
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    {    
        let query_exo = format!("INSERT INTO {} (exid, name, date, starter) VALUES (?1, ?2, ?3, ?4)", 
            TABLE_EXERCISES);
        let mut stmt_exo = tx.prepare(&query_exo)?;
        
        let query_reps = format!("INSERT INTO {} (exid, repnum, weight) VALUES (?1, ?2, ?3)", 
            TABLE_REPS);
        let mut stmt_reps = tx.prepare(&query_reps)?;

        for exercise in &exercises {
            if !exercise.name.is_empty() && !exercise.reps.is_empty() {
                stmt_exo.execute(params![
                    exercise.exid,
                    exercise.name,
                    date.to_string(),
                    exercise.starter
                ])?;
                for &(num, weight) in &exercise.reps {
                    stmt_reps.execute(params![
                        exercise.exid, 
                        num, 
                        weight, 
                    ])?;
                }
            }
        }
    }
    tx.commit()?;

    Ok(())
}

fn get_first_id ()->u32{
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();
    let max_id: Option<u32> = conn.query_row(
        "SELECT MAX(exid) FROM exercises", 
        [], 
        |row| row.get(0)
    ).unwrap_or(Some(0));
    
    max_id.unwrap_or(0)
}

pub fn New() -> Element {
    static NEXT_ID: LazyLock<AtomicU32> = LazyLock::new(|| {
        AtomicU32::new(get_first_id())
    }); 
       
    fn get_next_id() -> u32 {
        NEXT_ID.fetch_add(1, Ordering::SeqCst)
    }

    let mut exercises = use_signal(|| vec![Exercise{
        exid: get_next_id(),
        name: String::new(),
        reps: vec![(0,0.0)],
        date: chrono::Local::now().date_naive(),
        starter: false
    }]);
    
    let mut save_error = use_signal(|| None::<String>);
    let mut save_success = use_signal(|| false);

    let mut selected_date = use_signal(|| chrono::Local::now().date_naive());
    let mut date_text = use_signal(|| String::new());

    let save_data_closure = move |_| {
        let current_exercises: Vec<Exercise> = exercises.read().clone();

        println!("Exos: {:?}", current_exercises);

        let valid_exercises: Vec<Exercise> = current_exercises
            .into_iter()
            .filter(|ex| !ex.name.is_empty() && !ex.reps.is_empty())
            .collect();
        
        if !valid_exercises.is_empty() {
            match save_data(valid_exercises.clone(), *selected_date.read()) {
                Ok(_) => {
                    save_error.set(None);
                    save_success.set(true);
                    exercises.set(vec![]);
                    println!("Données sauvegardées: {} exercices", valid_exercises.len());
                },
                Err(e) => {
                    save_error.set(Some(e.to_string()));
                    save_success.set(false);
                    println!("Erreur lors de l'enregistrement: {}", e);
                }
            }
        } else {
            save_error.set(Some("Aucun exercice valide à enregistrer".to_string()));
            save_success.set(false);
        }
    };

    let add_exercise = move |_| {
        let mut current_exercises = exercises.read().clone();
        current_exercises.push(Exercise{
            exid: get_next_id(),
            name: String::new(),
            reps: vec![(0,0.0)],
            date: chrono::Local::now().date_naive(),
            starter: false
        });
        exercises.set(current_exercises);
    };
    
    let mut update_exercise = move |id: u32, updated_exercise: Exercise| {
        let mut current_exercises = exercises.read().clone();
        if let Some(index) = current_exercises.iter().position(|ex| ex.exid == id) {
            current_exercises[index] = updated_exercise;
            exercises.set(current_exercises);
        }
    };
    
    let mut delete_exercise = move |id: u32| {
        
        let mut current_exercises = exercises.read().clone();
        if let Some(index) = current_exercises.iter().position(|ex| ex.exid == id) {
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
                    class: "flex flex-row gap-4 items-center mb-6 w-full max-w-4xl",
                   
                    button {
                        class: "px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                        onclick: add_exercise,
                        "Ajouter un exercice"
                    }

                    input {
                        class: "px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                        r#type: "date",
                        value: "{selected_date.read().format(\"%Y-%m-%d\")}",
                        oninput: move |event| {
                            date_text.set(event.value().to_string());
                            if let Ok(date) = NaiveDate::parse_from_str(&event.value(), "%Y-%m-%d") {
                                selected_date.set(date);
                            }
                        }
                    }
                    
                    button {
                        class: "px-6 py-3 bg-blue-500 text-white font-medium rounded-lg transition-colors duration-200 hover:bg-blue-600",
                        onclick: save_data_closure,
                        "Enregistrer"
                    }
                }
                           
                div {
                    class: "grid grid-cols-1 gap-6 w-full px-4",
                    key: "{exercises.read().len()}",
                    {exercises.read().iter().map(|ex_with_id| {
                        let id = ex_with_id.exid;
                        rsx! {
                            NewExo {
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