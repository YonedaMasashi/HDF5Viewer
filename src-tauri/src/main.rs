#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hdf5_keys;
mod arrow_dataframe;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hdf5_keys::get_hdf5_keys, 
            arrow_dataframe::read_hdf5_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
