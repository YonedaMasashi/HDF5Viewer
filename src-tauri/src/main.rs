#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hdf5_keys;
mod arrow_dataframe;
mod hdf5_attrs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hdf5_keys::get_hdf5_keys, 
            arrow_dataframe::read_hdf5_data,
            hdf5_attrs::read_hdf5_attributes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
