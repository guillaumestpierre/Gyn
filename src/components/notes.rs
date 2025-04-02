#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{db::get_db_connection, routes::routes::Route::Home};

fn fetch_data() -> Vec<(String, String)> {
    // This function remains mostly the same, but would need better error handling
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT date, note FROM training ORDER BY date").unwrap();
    let rows = stmt.query_map([], |row| {
        let date: String = row.get(0)?; 
        let note: String = row.get(1)?;
        Ok((date, note))
    }).unwrap();

    rows.map(|row| row.unwrap()).collect()
}

pub fn Notes() -> Element {
    // Use a resource instead of direct function call
    let data = use_resource(move || async move {
        fetch_data()
    });
    
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
                class: "flex-1 p-6 overflow-auto",
                
                h1 { class: "text-2xl font-bold mb-6", "Notes" }
                
                match data.read().as_deref() {
                    Some(notes) => {
                        if notes.is_empty() {
                            rsx! {
                                div {
                                    class: "bg-neutral-100 p-8 rounded-lg text-center",
                                    "Aucune note!"
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    class: "grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3",
                                    
                                    {notes.iter().map(|(date, note)| rsx! {
                                        div {
                                            class: "bg-cyan-50 rounded-lg p-4 shadow hover:shadow-md transition-shadow duration-200 overflow-hidden",
                                            
                                            div {
                                                class: "text-sm font-semibold text-neutral-600 mb-2",
                                                "{date}"
                                            }
                                            
                                            div {
                                                class: "text-lg break-words whitespace-normal",
                                                "{note}"
                                            }
                                        }
                                    })}
                                }
                            }
                        }
                        
                    }
                    None => rsx! {
                        div {
                            class: "flex justify-center items-center h-64",
                            "Loading notes..."
                        }
                    }
                }
            }
        }
    }
}