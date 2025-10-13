#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::routes::routes::Route;

pub fn Home() -> Element {
    let pages = vec![
        (Route::Stats {}, "Stats"),
        (Route::Weight {}, "Weight"),
        (Route::Hist {}, "Historique"),
        (Route::ExosList {}, "Exercices"),
    ];

    rsx! {
        div { 
            class: "flex h-screen bg-neutral-300", 
            div {
                class: "w-64 h-full py-5 px-5 bg-neutral-200",
                
                for (route, label) in pages.into_iter() {
                    div{
                        class: "mb-4",
                        button {
                            class: "w-full rounded-lg py-2 px-4 bg-cyan-50 text-lg font-bold text-left transition-colors duration-100 hover:bg-cyan-100",
                            onclick: move |_| {
                                let _ = navigator().push(route.clone());
                            },
                            "{label}"
                        }
                    }
                }
            }

            div {
                class: "flex flex-1 justify-center items-center",
                button {
                    class: "px-20 py-10 bg-yellow-300 text-9xl font-bold rounded-xl flex items-center justify-center leading-none transition-colors duration-100 hover:bg-yellow-400",
                    onclick: move|_| {
                        let _ = navigator().push(Route::New {});
                    },
                    "New"
                }
            }
        }
    }
}