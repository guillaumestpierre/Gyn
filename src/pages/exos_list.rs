#![allow(non_snake_case)]
use chrono::NaiveDate;
use dioxus::prelude::*;
use crate::db::get_db_connection;
use crate::models::r#const::{BODY_PARTS, TABLE_EXERCISES, TABLE_REPS};
use crate::models::training::Exercise;
use crate::routes::routes::Route::Home;
use crate::components::body_part_dropdown::BodyPartDropdown;

fn fetch_exercises_by_name(name: &str) -> Result<Vec<Exercise>, rusqlite::Error>{
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();
    let mut res_exos: Vec<Exercise> = Vec::new();

    let mut stmt_exo = conn.prepare(&format!("SELECT exid, name, date, starter FROM {} WHERE name = ?1",
        TABLE_EXERCISES))?;
    let exo_rows = stmt_exo.query_map([name], |row| {
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

pub fn ExosList() -> Element {
    let mut dropdown_open = use_signal(|| false);
    let mut selected_exercise = use_signal(|| String::new());
    let mut exercises = use_signal(|| Vec::<Exercise>::new());

    rsx! {
        div {
            class: "flex h-screen bg-neutral-300", 
            div {
                class: "w-64 h-full py-5 px-5 bg-neutral-200",
                
                button {
                    class: "w-full rounded-lg py-2 px-4 border bg-cyan-50 text-lg font-bold text-left transition-colors duration-100 hover:bg-cyan-100",
                    onclick: move |_| {
                        let _ = navigator().push(Home {});
                    },
                    "Home"
                }
            }
            
            div {
                class: "flex flex-1 flex-col",
                
                // Header
                div {
                    class: "w-full bg-neutral-300 px-6 py-4",
                    
                    div {
                        class: "flex items-center gap-6",

                        // Dropdown Menu
                        div {
                            class: "relative",
                            
                            button {
                                class: "px-4 py-3 bg-yellow-100 font-medium rounded-lg transition-colors hover:bg-yellow-200",
                                onclick: move |_| {
                                    dropdown_open.set(!dropdown_open());
                                },
                                "Body Parts ▾"
                            }
                            if dropdown_open() {
                                div {
                                    class: "fixed inset-0 z-40",
                                    onclick: move |_| {
                                        dropdown_open.set(false);
                                    }
                                }
                            }
                            // Dropdown Content
                            if dropdown_open() {
                                div {
                                    class: "absolute left-0 mt-2 w-64 bg-white rounded-lg shadow-xl z-50",
                                    
                                    for body_part in BODY_PARTS {
                                        BodyPartDropdown {
                                            body_part: body_part.clone(),
                                            on_select: move |exercise_name: String| {
                                                selected_exercise.set(exercise_name.clone());
                                                dropdown_open.set(false);
                                                match fetch_exercises_by_name(&exercise_name) {
                                                    Ok(exos) => exercises.set(exos),
                                                    Err(e) => println!("Error fetching exercises: {}", e)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Main content area
                div {
                    class: "flex flex-1 justify-center p-4 items-center",
                    if !selected_exercise().is_empty() {
                        div {
                            class: "w-full max-w-3xl p-6 bg-white rounded-lg shadow-md",
                            h2 {
                                class: "text-2xl font-bold mb-4",
                                "{selected_exercise} - {exercises.read().len()}"
                            }
                            div {
                                // Make only the inner list scrollable with a reasonable max height
                                class: "list-disc list-inside overflow-y-auto max-h-[60vh] pr-2",
                                for ex in exercises.read().iter() {
                                    div {
                                        class: "p-4 border rounded-lg",
                                        h3 {
                                            class: "text-xl font-semibold mb-2 flex items-center gap-2",
                                            "Date: {ex.date}"
                                            if ex.starter {
                                                span {
                                                    class: "inline-flex items-center rounded-full bg-blue-100 px-2 py-0.5 text-xs font-semibold text-blue-700",
                                                    "Starter"
                                                }
                                            }
                                        }
                                        ul {
                                            class: "list-disc list-inside",
                                            for (repnum, weight) in ex.reps.iter() {
                                                li { "{repnum} reps at {weight}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div {
                            class: "text-center",
                            h2 {
                                class: "text-3xl font-bold mb-4",
                                "Please select an exercise"
                            }
                            p {
                                class: "text-gray-600",
                                "Hover over body parts to see exercises"
                            }
                        }
                    }
                }
            }
        }
    }
}