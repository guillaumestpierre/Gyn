#![allow(non_snake_case)]
use chrono::Local;
use dioxus::prelude::*;
use crate::models::r#const::EXERCISE_LIST;
use crate::models::training::Exercise;

#[component]
pub fn Exo(
    exercise: Option<Exercise>,
    on_change: Option<EventHandler<Exercise>>,
    on_delete: Option<EventHandler<()>>,
) -> Element {
    let initial_exercise = exercise.unwrap_or_else(Exercise::new);

    let mut selected_exercise_name = use_signal(|| initial_exercise.name.clone());


    let mut reps = use_signal(|| {
        if initial_exercise.reps.is_empty() {
            vec![(0u32, 0f32)]
        } else {
            initial_exercise.reps.clone()
        }
    });

    let mut isStarter = use_signal(|| false);

    let mut rep_number = use_signal(|| reps.read().len());
    
    let update_exercises = move || {
        if let Some(handler) = &on_change {
            let exercise = Exercise {
                name: selected_exercise_name.read().to_string(),
                reps: reps.read().clone(),
                date: Local::now().naive_local().into(),
                starter: *isStarter.read(),
            };
            handler.call(exercise);
        }
    };

    let mut add_rep = move |_| {
        let mut updated_reps = reps.read().clone();
        updated_reps.push((0, 0.0));
        reps.set(updated_reps);
        rep_number.set(reps.read().len());
        update_exercises();
    };
    
    let mut delete_rep = move |rep_index: usize| {
        let mut updated_reps = reps.read().clone();
        if rep_index < updated_reps.len() {
            updated_reps.remove(rep_index);
            if updated_reps.is_empty() {
                updated_reps.push((0, 0.0));
            }
            reps.set(updated_reps);
            rep_number.set(reps.read().len());
            update_exercises();
        }
    };
    
    let delete_exercise = move |_| {
        if let Some(handler) = &on_delete {
            handler.call(());
        }
    };

    rsx! {
        div {
            class: "flex flex-col bg-neutral-100 p-4 rounded-lg shadow-sm gap-3 w-full max-w-lg",
            
            div {
                class: "flex flex-row justify-between items-center",
                
                select {
                    class: "px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 flex-grow",
                    onchange: move |event| {
                        selected_exercise_name.set(event.value().to_string());
                        update_exercises();
                    },
                    option { 
                        value: "", 
                        disabled: true, 
                        selected: selected_exercise_name.read().is_empty(), 
                        "Sélectionner un exercice" 
                    },
                    {EXERCISE_LIST.iter().map(|exercise_name| {
                        let name = exercise_name.to_string();
                        rsx! {
                            option {
                                key: "{name}",
                                value: "{name}",
                                selected: *selected_exercise_name.read() == name,
                                "{name}"
                            }
                        }
                    })}
                }

                div {
                    class: "flex items-center ml-2",
                    label {
                        class: "inline-flex items-center cursor-pointer",
                        input {
                            class: "sr-only peer",
                            r#type: "checkbox",
                            checked: *isStarter.read(),
                            oninput: move |event| {
                                isStarter.set(event.value().parse().unwrap_or(false));
                                update_exercises();
                            }
                        }
                        div {
                            class: "relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600",
                        }
                        span {
                            class: "ml-2 text-sm font-medium text-gray-700",
                            "Starter"
                        }
                    }
                }
                
                button {
                    class: "ml-2 px-3 py-2 border bg-red-50 rounded-lg hover:bg-red-100 focus:outline-none focus:ring-2 focus:ring-red-500",
                    onclick: delete_exercise,
                    "✕"
                }
            }
            
            div {
                class: "flex justify-end mt-2",
                button {
                    class: "px-4 py-2 bg-blue-50 border border-blue-200 rounded-lg hover:bg-blue-100 focus:outline-none focus:ring-2 focus:ring-blue-500",
                    onclick:add_rep,
                    "Ajouter une répétition"
                }
            }
            
            div {
                class: "flex flex-col gap-2 mt-2",
                
                div {
                    class: "flex flex-row items-center px-2 text-sm font-medium text-gray-600",
                    div { class: "flex-1", "Répétitions" }
                    div { class: "flex-1", "Poids (kg)" }
                    div { class: "w-10" }
                }
                
                {reps.read().iter().enumerate().map(|(rep_index, &(num, weight))| {
                        rsx! {
                            div {
                                key: "{rep_index}",
                                class: "flex flex-row items-center gap-2",
                                
                                input {
                                    class: "flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    r#type: "number",
                                    min: "1",
                                    placeholder: "Nombre",
                                    value: "{num}",
                                    oninput: move |event| {
                                        if let Ok(new_num) = event.value().parse::<u32>() {
                                            let mut updated_reps = reps.read().clone();
                                            updated_reps[rep_index].0 = new_num;
                                            reps.set(updated_reps);
                                            update_exercises();
                                            
                                        }
                                    }
                                }
                                
                                input {
                                    class: "flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    r#type: "number",
                                    min: "0",
                                    step: "0.5",
                                    placeholder: "Poids",
                                    value: "{weight}",
                                    oninput: move |event| {
                                        if let Ok(new_weight) = event.value().parse::<f32>() {
                                            let mut updated_reps = reps.read().clone();
                                            updated_reps[rep_index].1 = new_weight;
                                            reps.set(updated_reps);
                                            update_exercises();
                                        }
                                    }
                                }
                                
                                button {
                                    class: "w-10 h-10 flex items-center justify-center border bg-red-50 rounded-lg hover:bg-red-100 focus:outline-none focus:ring-2 focus:ring-red-500",
                                    onclick: move |_| delete_rep(rep_index),
                                    "✕"
                                }
                            }
                        }
                    })
                }
            }
        }
    }
}