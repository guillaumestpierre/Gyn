#![allow(non_snake_case)]
use chrono::NaiveDate;
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
        let mut stmt = tx.prepare("INSERT INTO exercise (name, num, weight, date, starter) VALUES (?1, ?2, ?3, ?4, ?5)")?;

        for exercise in &exercises {
            // Only save exercises that have a name and at least one rep
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

    #[derive(Clone, Debug, PartialEq)]
    struct ExerciseWithId {
        id: usize,
        exercise: Exercise,
    }

    static mut NEXT_ID: usize = 0;
    
    fn get_next_id() -> usize {
        unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        }
    }

    let mut exercises = use_signal(|| vec![ExerciseWithId {
        id: get_next_id(),
        exercise: Exercise::new(),
    }]);
    
    let mut save_error = use_signal(|| None::<String>);
    let mut save_success = use_signal(|| false);

    let mut save_data_closure = move |_| {
        let current_exercises: Vec<Exercise> = exercises.read()
            .iter()
            .map(|ex_with_id| ex_with_id.exercise.clone())
            .collect();

        let valid_exercises: Vec<Exercise> = current_exercises
            .into_iter()
            .filter(|ex| !ex.name.is_empty() && !ex.reps.is_empty())
            .collect();
        
        if !valid_exercises.is_empty() {
            match save_data(valid_exercises.clone()) {
                Ok(_) => {
                    save_error.set(None);
                    save_success.set(true);
                    exercises.set(vec![ExerciseWithId {
                        id: get_next_id(),
                        exercise: Exercise::new(),
                    }]);
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
        current_exercises.push(ExerciseWithId {
            id: get_next_id(),
            exercise: Exercise::new(),
        });        exercises.set(current_exercises);
    };
    
    let mut update_exercise = move |id: usize, updated_exercise: Exercise| {
        let mut current_exercises = exercises.read().clone();
        if let Some(index) = current_exercises.iter().position(|ex| ex.id == id) {
            current_exercises[index].exercise = updated_exercise;
            exercises.set(current_exercises);
        }
    };
    
    let mut delete_exercise = move |id: usize| {
        
        let mut current_exercises = exercises.read().clone();
        if let Some(index) = current_exercises.iter().position(|ex| ex.id == id) {
            current_exercises.remove(index);          
            exercises.set(current_exercises);
        } else {
            println!("Exercice avec ID {} non trouvé", id);
        }
        println!("aprèsm del {:?}", exercises);
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
                        let id = ex_with_id.id;
                        rsx! {
                            Exo {
                                key: "{id}",
                                exercise: Some(ex_with_id.exercise.clone()),
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