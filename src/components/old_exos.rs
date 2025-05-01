#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::models::training::Exercise;

#[component]
pub fn OldExo(
    exercise: Option<Exercise>,
) -> Element {

    let initial_exercise = exercise.unwrap_or_else(Exercise::new);
    println!("{:?}", initial_exercise);
        
    let reps = use_signal(|| {
        if initial_exercise.reps.is_empty() {
            vec![(0u32, 0f32)]
        } else {
            initial_exercise.reps.clone()
        }
    });
    
    rsx! {
        div {
            class: "flex flex-col bg-neutral-100 p-6 rounded-lg shadow-md gap-4 w-full",
            div {
                class: "text-base text-gray-600",
                if initial_exercise.starter {
                    "Starter"
                } else {
                    "Normal"
                }
            }
            div {
                class: "flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-0 text-base",
                "{initial_exercise.date}",
            }

            div {
                class: "flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-0 text-base",
                "{initial_exercise.name}"
            }
            div {
                class: "flex flex-col gap-2 mt-2",
                
                div {
                    class: "flex flex-row items-center px-2 text-sm font-medium text-gray-600 mb-2",
                    div { class: "flex-1 px-1", "Répétitions" }
                    div { class: "flex-1 px-1", "Poids (kg)" }
                    div { class: "w-10" }
                }
                
                {reps.read().iter().enumerate().map(|(rep_index, &(num, weight))| {
                    rsx! {
                        div {
                            key: "{rep_index}",
                            class: "flex flex-row items-center gap-3 mb-2",
                            
                            div {
                                class: "flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-0 text-base",
                                "{num}",
                            }
                            
                            div {
                                class: "flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-0 text-base",
                                "{weight}",
                            }
                        }
                    }
                })}
            }
        }
    }
}