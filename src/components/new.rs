#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::models::training::{Exercise, Training};
use crate::routes::routes::Route::Home;

pub fn New() -> Element {
    rsx! {
        div {
            class: "flex h-screen bg-neutral-300",   
            div {
                class: "py-5 px-5 grid grid-cols-1 grid-rows-5 gap-4 w-1/8 h-1/3",
                
                button {
                    class: "w-full h-full row-span-1 border-black mb-1 px-10 bg-cyan-50 text-2xl font-bold text-left fontsize-lg rounded-lg transition-colors duration-100 hover:bg-cyan-100",
                    style: "width: 200px; height: 36px",
                    onclick: move |_| {
                        let _ = navigator().push(Home {});
                    },
                    "Home"
                }
            }
            div {
                class: "flex flex-1 justify-center items-center",
                
            }
        }
    }
}