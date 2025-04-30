#![allow(non_snake_case)]
use dioxus::prelude::*;
use rusqlite::params;
use crate::components::exo::Exo;
use crate::models::training::Exercise;
use crate::routes::routes::Route::Home;
use crate::db::get_db_connection;

fn save_data(exercises: Vec<Exercise>) -> rusqlite::Result<()> {
    let conn = get_db_connection();
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    {    
        let mut stmt = tx.prepare("INSERT INTO exercise (name, num, weight) VALUES (?1, ?2, ?3)")?;

        for exercise in &exercises {
            if !exercise.name.is_empty() && !exercise.reps.is_empty() {
                for &(num, weight) in &exercise.reps {
                    stmt.execute(params![
                        exercise.name, 
                        num, 
                        weight, 
                        exercise.date.to_string(), 
                        exercise.starter
                    ])?;
                }
            }
        }
    }
    tx.commit()?;

    Ok(())
}

pub fn New() -> Element {
    let mut exercises = use_signal(|| vec![Exercise::new()]);
    let mut save_error = use_signal(|| None::<String>);
    let mut save_success = use_signal(|| false);

    let mut save_data_closure = move |_| {
        let current_exercises = exercises.read().clone();
        let valid_exercises: Vec<Exercise> = current_exercises
            .into_iter()
            .filter(|ex| !ex.name.is_empty() && !ex.reps.is_empty())
            .collect();

        if !valid_exercises.is_empty() {
            match save_data(valid_exercises.clone()) {
                Ok(_) => {
                    save_error.set(None);
                    save_success.set(true);
                    exercises.set(vec![Exercise::new()]);
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

    let mut add_exercise = move |_| {
        let mut current_exercises = exercises.read().clone();
        current_exercises.push(Exercise::new());
        exercises.set(current_exercises);
    };
    
    let mut update_exercise = move |index: usize, updated_exercise: Exercise| {
        let mut current_exercises = exercises.read().clone();
        if index < current_exercises.len() {
            current_exercises[index] = updated_exercise;
            exercises.set(current_exercises);
        }
    };
    
    
    let mut delete_exercise = move |index: usize| {
        println!("Tentative de suppression de l'exercice à l'index: {}", index);
        let filtered_exercises: Vec<Exercise> = exercises
            .read()
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, ex)| ex.clone())
            .collect();
        
        println!("Nouvelle liste d'exercices (taille: {}): {:?}", filtered_exercises.len(), &filtered_exercises);
        if filtered_exercises.is_empty() {
            exercises.set(vec![Exercise::new()]);
        } else {
            exercises.set(filtered_exercises);
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
                class: "flex flex-1 flex-col align-top items-center space-y-6",
                
                div {
                    class: "flex flex-row gap-4 items-center",               
                    button {
                        class: "px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                        onclick: add_exercise,
                        "Ajouter un exercice"
                    }
                    
                    button {
                        class: "px-6 py-3 bg-blue-500 text-white font-medium rounded-lg transition-colors duration-200 hover:bg-blue-600",
                        onclick: save_data_closure,
                        "Enregistrer"
                    }
                }

                div {
                    class: "grid grid-cols-3 gap-4 w-full max-w-lg justify-left align-left",
                    {exercises.read().iter().enumerate().map(|(index, exercise)| {
                        let current_index = index;
                        let exercise_clone = exercise.clone();
                        rsx! {
                            Exo {
                                key: "{index}",
                                exercise: Some(exercise_clone),
                                on_change: move |updated_exercise| update_exercise(current_index, updated_exercise),
                                on_delete: move |_| delete_exercise(current_index)
                            }
                        }
                    })}
                }
            }
        }
    }
}