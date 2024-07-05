use hdf5::File;
use hdf5::types::{TypeDescriptor, VarLenUnicode};
use serde::{Serialize, Deserialize};
use tauri::command;

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    name: String,
    value: String,
}

#[command]
pub fn read_hdf5_attributes(file_path: String, full_key: String) -> Result<Vec<Attribute>, String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    
    let mut attributes = Vec::new();

    let dataset = file.dataset(&full_key);

    match dataset {
        Ok(ds) => {
            println!("read_hdf5_attributes : {:?}, ndim:{:?}", ds.name(), ds.ndim());

            for attr_name in ds.attr_names().unwrap() {
                let attr = ds.attr(&attr_name).unwrap();
                let dtype = attr.dtype().unwrap();

                let mut value = "".to_string();

                match dtype.to_descriptor().unwrap() {
                    TypeDescriptor::VarLenUnicode => {
                        println!("read_hdf5_attributes : TypeDescriptor::VarLenUnicode");

                        match attr.read_scalar::<VarLenUnicode>() {
                            Ok(data) => {
                                value = data.as_str().to_string();
                            },
                            Err(err) => {
                                println!("convert_string - ERR: {:?}", err.to_string());
                            }
                        }
                    }
                    _ => {
                        println!("Unsupported attribute type.");
                    }
                }
                
                attributes.push(Attribute { name: attr_name, value: value });

            }
        },
        Err(_) => {
        }
    }
        
    Ok(attributes)
}
