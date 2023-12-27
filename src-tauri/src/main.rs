// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use stardata::celestial_object_dao::fetch_celestial_objects;

mod stardata;

#[derive(Debug, Serialize, Deserialize)]
struct Cartesian {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct StarResp {
    cartesian: Cartesian,
    color: String,
    temperature: f64,
}

#[tauri::command]
fn stars() -> String {
    let stars = fetch_celestial_objects(100).unwrap();

    let mut stars_res: Vec<StarResp> = vec![];

    for star in stars.iter() {
        stars_res.push(StarResp {
            cartesian: Cartesian {
                x: star.get_cartesian_x(),
                y: star.get_cartesian_y(),
                z: star.get_cartesian_z(),
            },
            color: star.get_color_in_hex(),
            temperature: star.get_temp_in_kelvin().round(),
        })
    }

    serde_json::to_string(&stars_res).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![stars])
        .run(tauri::generate_context!())
        .expect("error while running starchart application");
}
