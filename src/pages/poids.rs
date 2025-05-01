#![allow(non_snake_case)]
use dioxus::prelude::*;
use chrono::NaiveDate;
use crate::routes::routes::Route::Home;
use rusqlite::{params, Result};
use dioxus_charts::LineChart;

use crate::db::get_db_connection;

fn save_data(data: (f32, NaiveDate)) -> Result<()> {
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();
    
    conn.execute(
        "INSERT INTO weight (weight, date) VALUES (?1, ?2)",
        params![data.0, data.1.to_string()],
    )?;
    
    Ok(())
}

fn delete_data(data: (f32, String)) -> Result<()> {
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();
    
    conn.execute(
        "DELETE FROM weight WHERE weight = (?1) AND date = (?2)",
        params![data.0, data.1.to_string()],
    )?;
    Ok(())
}

fn fetch_data() -> Result<Vec<(NaiveDate, f32)>> {
    let conn = get_db_connection();
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT date, weight FROM weight ORDER BY date")?;
    let rows = stmt.query_map([], |row| {
        let date: String = row.get(0)?; 
        let weight: f32 = row.get(1)?;
        Ok((NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap(), weight))
    })?;

    Ok(rows.collect::<Result<Vec<_>>>()?)
}

pub fn Poids() -> Element {
    let mut weight_history = use_signal(|| fetch_data().unwrap());
    let data = weight_history.read().clone();

    let chart_serie: Vec<f32> = data.iter()
        .map(|(_, weight)| (*weight as f32))
        .collect();
    let chart_label: Vec<String> = data.iter()
        .map(|(date, _)| (date.format("%Y-%m-%d").to_string()))
        .collect();

    let mut weight_text = use_signal(|| String::new());
    let mut weight = use_signal(|| 0f32);
    let mut weight_to_del = use_signal(|| 0f32);
    
    let mut date_text = use_signal(|| String::new());
    let mut selected_date = use_signal(|| chrono::Local::now().date_naive());
    let mut date_to_del = use_signal(|| String::new());
    
    let mut weight_data = use_signal(|| None::<(f32, NaiveDate)>);
    let mut save_error = use_signal(|| None::<String>);
    
    let save_data_closure = move |_| {
        if *weight.read() > 0.0 {
            let add_data = (*weight.read(), *selected_date.read());
            
            match save_data(add_data) {
                Ok(_) => {
                    weight_data.set(Some(add_data));
                    save_error.set(None);
                    weight_text.set("".to_string());
                    weight_history.set(fetch_data().unwrap());
                    println!("Données sauvegardées: Poids={}, Date={}", add_data.0, add_data.1);
                },
                Err(e) => {
                    save_error.set(Some(e.to_string()));
                    println!("Erreur lors de l'enregistrement: {}", e);
                }
            }
        }
    };

    let mut delete_data_closure = move || {
        let del_data = (*weight_to_del.read(), date_to_del.read().clone());
        match delete_data(del_data) {
            Ok(_) => {
                save_error.set(None);
                weight_history.set(fetch_data().unwrap());
                println!("Donnée supprimée");
            },
            Err(e)=>{
                save_error.set(Some(e.to_string()));
                println!("Erreur lors de la suppression: {}", e);
            }
        }
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
                class: "flex flex-1 flex-col justify-center items-center space-y-6",

                LineChart{
                    series: vec![chart_serie.clone()],
                    labels: chart_label,
                    width: "140%",
                    height: "140%",
                    lowest: 150.0,
                    highest: 180.0,
                    show_dots: false
                }
                div {
                    class: "flex flex-none pt-[120px] pl-[150px] flex-row space-x-6",
                    
                    div {
                        class: "flex flex-row gap-4",
                        input {
                            class: "px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                            placeholder: "Poids (lbs)",
                            value: "{weight_text}",
                            oninput: move |event| {
                                weight_text.set(event.value().to_string());
                                if let Ok(num) = event.value().parse::<f32>() {
                                    weight.set(num);
                                }
                            }
                        }
                        input {
                            class: "px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "date",
                            value: "{selected_date.read().format(\"%Y-%m-%d\")}",
                            oninput: move |event| {
                                date_text.set(event.value().to_string());
                                if let Ok(date) = NaiveDate::parse_from_str(&event.value(), "%Y-%m-%d") {
                                    selected_date.set(date);
                                }
                            }
                        }
                        button {
                            class: "px-6 py-3 bg-blue-500 text-white font-medium rounded-lg transition-colors duration-200 hover:bg-blue-600",
                            onclick: save_data_closure,
                            "Enregistrer"
                        }
                    }
                }
            }
            div {
                class:"grid grid-cols-1 content-start py-5 px-5 h-110 overflow-y-auto",
                {data.iter().rev().map(|(date, weight)| {
                    let date = date.to_owned();
                    let weight = *weight;
                    rsx! {
                        button {
                            class: "bg-cyan-50 rounded-lg p-4 mb-4 h-24 shadow hover:bg-red-200 transition-colors duration-200 overflow-hidden",
                            onclick: move |_|{
                                weight_to_del.set(weight);
                                date_to_del.set(date.to_string());
                                delete_data_closure();
                            },
                            div {
                                class: "text-sm font-semibold text-neutral-600 mb-2",
                                "{date}"
                            }
                            
                            div {
                                class: "text-lg break-words whitespace-normal",
                                "{weight}"
                            }
                        }
                    }
                })}
            }
        }
    }
}