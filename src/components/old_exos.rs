#![allow(non_snake_case)]
use chrono::NaiveDate;
use dioxus::prelude::*;
use crate::models::{r#const::EXERCISE_LIST, training::Exercise};

// TODO: fix UI, copy new_exos UI when modifying, what for save button to delete exo
#[component]
pub fn OldExo(
    exercise: Option<Exercise>,
    on_change: Option<EventHandler<Exercise>>,
    on_delete: Option<EventHandler<()>>,
) -> Element {

    let initial_exercise = exercise.unwrap_or_else(Exercise::new);

    let mut to_delete = use_signal(||false);

    let mut name = use_signal(|| initial_exercise.name.clone());
    let mut reps = use_signal(|| {
        if initial_exercise.reps.is_empty() {
            vec![(0u32, 0f32)]
        } else {
            initial_exercise.reps.clone()
        }
    });
    let mut date = use_signal(|| initial_exercise.date);
    let mut starter = use_signal(||initial_exercise.starter);
    
    let mut edit_state = use_signal(||false);

    let delete_exercise = move || {
        if let Some(handler) = &on_delete {
            handler.call(());
        }
    };

    let save_data_closure = move||{
        if *to_delete.read(){
            delete_exercise();
        }
        else if let Some(handler) = &on_change{
            let exercise = Exercise {
                exid: initial_exercise.exid,
                name: name.to_string(),
                reps: reps.read().clone(),
                date: *date.read(),
                starter: *starter.read(),
            };
            handler.call(exercise);
        }
        
    };

    let mut delete_rep = move |rep_index: usize| {
        let mut updated_reps = reps.read().clone();
        if rep_index < updated_reps.len() {
            updated_reps.remove(rep_index);
            
            if updated_reps.is_empty() {
                updated_reps.push((0, 0.0));
            }
            
            reps.set(updated_reps);
        }
    };

    let mut add_rep = move || {
        let mut updated_reps = reps.read().clone();
        updated_reps.push((0, 0.0));
        reps.set(updated_reps);
    };
   
    rsx! {
        div {
            class: {
                let base_class = "flex flex-col p-6 rounded-lg shadow-md gap-4 w-full";
                if *to_delete.read() {
                    format!("{} bg-red-100", base_class)
                } else {
                    format!("{} bg-neutral-100", base_class)
                }
            },
            div {
                label {
                    class: "inline-flex items-center cursor-pointer",
                    input {
                        class: "sr-only peer",
                        r#type: "checkbox",
                        disabled: match !*edit_state.read(){
                            true => {true},
                            false => {false}
                        },
                        checked: *starter.read(),
                        oninput: move |event| {
                            starter.set(event.value().parse().unwrap_or(false));
                        }
                    }
                    div {
                        class: "relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600",
                    }
                    span {
                        class: "ml-2 text-sm font-medium text-gray-700",
                        if *starter.read() {
                            "Starter"
                        } else {
                            "Normal"
                        }
                    }
                }
                
                button{
                    class:"px-6 py-3 bg-blue-500 text-white justify-right rounded-lg transition-colors duration-200 hover:bg-blue-600",
                    onclick: move|_|{
                        let current_state = *edit_state.read();
                        if current_state {
                            save_data_closure();
                        }
                        edit_state.set(!current_state);
                    },
                    if *edit_state.read() {
                        "Enregistrer"
                    } else {
                        "Modifier"
                    }
                }
                button {
                    class: "w-10 h-10 flex-shrink-0 flex items-center justify-center border bg-red-50 rounded-lg hover:bg-red-100 focus:outline-none focus:ring-2 focus:ring-red-500",
                    disabled: match !*edit_state.read(){
                        true => {true},
                        false => {false}
                    },
                    onclick: move |_| {
                        let current_to_delete = *to_delete.read();
                        to_delete.set(!current_to_delete);
                    },
                    "✕"
                }
            }
            input {
                class: "flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-0 text-base",
                r#type: "date",
                value: "{initial_exercise.date.format(\"%Y-%m-%d\")}",
                disabled: match !*edit_state.read(){
                    true => {true},
                    false => {false}
                },
                oninput: move |event| {
                    if let Ok(new_date) = NaiveDate::parse_from_str(&event.value(), "%Y-%m-%d") {
                        date.set(new_date);
                    }
                }         
            }

            select {   
                class: "px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 flex-grow",
                    value: "{name}",
                    disabled: match !*edit_state.read(){
                        true => {true},
                        false => {false}
                    },
                    onchange: move |event| {
                        name.set(event.value().to_string());
                    }, 
                option { 
                    value: "", 
                    disabled: true,
                    selected: name.read().is_empty(), 
                    "Sélectionner un exercice" 
                },
                {EXERCISE_LIST.iter().map(|exercise| {
                    let exercise_name = exercise.to_string();
                    rsx! {
                        option {
                            key: "{exercise_name}",
                            value: "{exercise_name}",
                            selected: *name.read() == exercise_name,
                            "{exercise_name}"
                        }
                    }
                })}
            }
            div {
                class: "flex flex-col gap-2 mt-2",
                
                div {
                    class: "flex flex-row items-center px-2 text-sm font-medium text-gray-600 mb-2",
                    div { class: "flex-1 px-1", "Répétitions" }
                    div { class: "flex-1 px-1", "Poids (kg)" }
                    div { class: "w-10" }
                    button {
                        class: "w-10 h-10 flex-shrink-0 flex items-center justify-center border bg-blue-50 rounded-lg hover:bg-blue-100 focus:outline-none focus:ring-2 focus:ring-blue-500",
                        disabled: match !*edit_state.read(){
                            true => {true},
                            false => {false}
                        },
                        onclick: move |_| add_rep(),
                        "+"
                    }
                }
                
                {reps.read().iter().enumerate().map(|(rep_index, &(num, weight))| {
                    rsx! {
                        div {
                            key: "{rep_index}",
                            class: "flex flex-row items-center gap-3 mb-2",
                            
                            input {
                                class: "flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-0 text-base",
                                r#type: "number",
                                min: 1,
                                disabled: match !*edit_state.read(){
                                    true => {true},
                                    false => {false}
                                },
                                value: "{num}",
                                oninput: move |event| {
                                    if let Ok(new_num) = event.value().parse::<u32>() {
                                        let mut updated_reps = reps.read().clone();
                                        updated_reps[rep_index].0 = new_num;
                                        reps.set(updated_reps);
                                    }
                                }
                            }
                            
                            input {
                                class: "flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-0 text-base",
                                r#type: "number",
                                min: 0,
                                step: 0.5,
                                disabled: match !*edit_state.read(){
                                    true => {true},
                                    false => {false}
                                },
                                value: "{weight}",
                                oninput: move |event| {
                                    if let Ok(new_weight) = event.value().parse::<f32>() {
                                        let mut updated_reps = reps.read().clone();
                                        updated_reps[rep_index].1 = new_weight;
                                        reps.set(updated_reps);
                                    }
                                }
                            }

                            button {
                                class: "w-10 h-10 flex-shrink-0 flex items-center justify-center border bg-red-50 rounded-lg hover:bg-red-100 focus:outline-none focus:ring-2 focus:ring-red-500",
                                disabled: match !*edit_state.read(){
                                    true => {true},
                                    false => {false}
                                },
                                onclick: move |_| delete_rep(rep_index),
                                "✕"
                            }
                        }
                    }
                })}
            }
        }
    }
}