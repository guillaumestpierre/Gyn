#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::models::r#const::BODY_PARTS;
use crate::routes::routes::Route::Home;
use crate::components::body_part_dropdown::BodyPartDropdown;

pub fn ExosList() -> Element {
    let mut dropdown_open = use_signal(|| false);

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
                class: "flex flex-1 flex-col",
                
                // Header
                div {
                    class: "w-full bg-neutral-300 text-white px-6 py-4",
                    
                    div {
                        class: "flex items-center gap-6",

                        // Dropdown Menu
                        div {
                            class: "relative",
                            
                            button {
                                class: "rounded-lg py-2 px-6 text-white font-semibold transition-colors",
                                onclick: move |_| {
                                    dropdown_open.set(!dropdown_open());
                                },
                                "Select Body Part ▾"
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Main content area
                div {
                    class: "flex flex-1 justify-center items-center",
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