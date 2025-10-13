#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::models::body_part::BodyPart;

#[component]
pub fn BodyPartDropdown(body_part: BodyPart, on_select: EventHandler<String>) -> Element {
    let mut is_hovered = use_signal(|| false);

    rsx! {
        div {
            class: "relative group",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            
            // Body Part Header
            div {
                class: "px-4 py-3 hover:bg-gray-100 cursor-pointer font-semibold text-gray-700 border-b",
                "{body_part.name}"
            }

            // Exercise List (shown on hover)
            if is_hovered() {
                div {
                    class: "absolute left-full top-0 ml-2 w-64 bg-white rounded-lg shadow-xl border z-50 max-h-96 overflow-y-auto",
                    
                    for exercise in body_part.exercises.iter() {
                        div {
                            class: "px-4 py-2 hover:bg-gray-50 cursor-pointer text-sm text-gray-700 border-b last:border-b-0",
                            onclick: {
                                let exercise = exercise.to_string();
                                move |_| {
                                    on_select.call(exercise.clone());
                                }
                            },
                            "{exercise}"
                        }
                    }
                }
            }
        }
    }
}