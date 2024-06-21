use arrow::array::{Int32Array, Int64Array, RecordBatch, StringArray};
use arrow::json::writer::JsonFormat;
use ndarray::Data;
use polars::io::{json, ArrowReader};
use polars::prelude::*;
use polars_arrow::array::Utf8Array;
use serde::Serialize;
use tauri::command;
use std::path::PathBuf;

use std::io::Cursor;
use std::result;
use arrow::ipc::reader::FileReader;
use arrow::util::pretty;
use polars::frame::DataFrame as PolarsDataFrame;

use serde_json::json;

use polars::prelude::*;

use hdf5::types::{TypeDescriptor, VarLenUnicode};
use hdf5::File;
use hdf5::Group;
use hdf5::Dataset;
use hdf5::H5Type;

use ndarray::Array1;


#[derive(Serialize)]
struct DataFrameRow {
    column1: String,
    column2: i32,
}

#[derive(Serialize)]
struct DataFrameRow2 {
    column1: i64,
    column2: String,
    column3: String,
}

#[command]
fn get_dataframe() -> Vec<DataFrameRow> {
    // DataFrame を作成
    let df = df!(
        "column1" => &["A", "B", "C"],
        "column2" => &[1, 2, 3]
    ).expect("Failed to create DataFrame");

    // DataFrame を Vec<DataFrameRow> に変換
    let column1_series = df.column("column1").expect("Column not found");
    let column2_series = df.column("column2").expect("Column not found");

    let mut rows = Vec::new();

    for i in 0..df.height() {
        let column1_value = column1_series.get(i).unwrap().to_string();
        let column2_value = match column2_series.get(i).unwrap() {
            AnyValue::Int32(val) => val,
            _ => panic!("Unexpected type in column2"),
        };
        rows.push(DataFrameRow {
            column1: column1_value,
            column2: column2_value,
        });
    }

    rows
}


#[command]
fn read_data_frame(file_path: String) -> Vec<DataFrameRow2> {
    let path = PathBuf::from(file_path.clone());
    println!("Received file path: {:?}", path);

    // HDF5 を読み込む
    let file = File::open(file_path.clone()).unwrap();
    let dataset = file.dataset("arrow_data").unwrap();

    // データを読み込む
    let data: Vec<u8> = dataset.read_raw().unwrap();

    // Arrow IPC 形式のデータを読み込む
    let cursor = Cursor::new(data);
    let reader = FileReader::try_new(cursor, None).unwrap();

    // Arrow のレコードバッチを Polars の DataFrame に変換
    let mut batches = Vec::new();
    for batch in reader {
        let record_batch = batch.unwrap();

        let columns: Vec<Series> = record_batch
            .columns()
            .iter()
            .zip(record_batch.schema().fields())
            .map(|(array, field)| {
                match field.data_type() {
                    arrow::datatypes::DataType::Int64 => {
                        let int_array = array
                            .as_any()
                            .downcast_ref::<Int64Array>()
                            .expect("Failed to downcast to Int64Array");
                        Series::new(field.name(), int_array.values())
                    },
                    arrow::datatypes::DataType::Utf8 => {
                        let str_array = array
                            .as_any()
                            .downcast_ref::<StringArray>()
                            .expect("Failed to downcast to Utf8Array");
                        let str_values: Vec<_> = str_array.iter().map(|s| s.map(|s_val| s_val)).flatten().collect();
                        Series::new(field.name(), &str_values)
                    }
                    _ => unimplemented!("Unsupported data type:{}", field.data_type()),
                }
            })
            .collect();
        let df = DataFrame::new(columns).unwrap();
        batches.push(df);
    }

    let mut concatenated_df = batches[0].clone();
    for df in &batches[1..] {
        concatenated_df.vstack_mut(&df);
    }

    // DataFrame を Vec<DataFrameRow2> に変換
    let column1_series = concatenated_df.column("column1").expect("Column not found");
    let column2_series = concatenated_df.column("column2").expect("Column not found");
    let column3_series = concatenated_df.column("column3").expect("Column not found");

    let mut rows = Vec::new();

    for i in 0..concatenated_df.height() {
        let column1_value = match column1_series.get(i).unwrap() {
            AnyValue::Int64(val) => val,
            _ => panic!("Unexpected type in column1"),
        };
        let column2_value = column2_series.get(i).unwrap().to_string();
        let column3_value = column3_series.get(i).unwrap().to_string();
        rows.push(DataFrameRow2 {
            column1: column1_value,
            column2: column2_value,
            column3: column3_value,
        });
    }
   
    rows
}

// - 2024.06.14 -------------------------------------------------------------------------------------------------------
#[derive(Serialize)]
struct DataFrameRow3 {
    #[serde(flatten)]
    row: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize)]
struct DataFrameSchema {
    name: String,
    dtype: String,
}

#[derive(Serialize)]
struct DataFrameResponse {
    schema: Vec<DataFrameSchema>,
    data: Vec<DataFrameRow3>,
}

fn get_dataframe_dynamic_local(dataset: &Dataset) -> Result<String, String> {
    // データを読み込む
    let data: Vec<u8> = dataset.read_raw().unwrap();

    // Arrow IPC 形式のデータを読み込む
    let cursor = Cursor::new(data);
    let reader = FileReader::try_new(cursor, None).unwrap();

    // Arrow のレコードバッチを Polars の DataFrame に変換
    let mut batches = Vec::new();
    for batch in reader {
        let record_batch = batch.unwrap();

        let columns: Vec<Series> = record_batch
            .columns()
            .iter()
            .zip(record_batch.schema().fields())
            .map(|(array, field)| {
                match field.data_type() {
                    arrow::datatypes::DataType::Int64 => {
                        let int_array = array
                            .as_any()
                            .downcast_ref::<Int64Array>()
                            .expect("Failed to downcast to Int64Array");
                        Series::new(field.name(), int_array.values())
                    },
                    arrow::datatypes::DataType::Utf8 => {
                        let str_array = array
                            .as_any()
                            .downcast_ref::<StringArray>()
                            .expect("Failed to downcast to Utf8Array");
                        let str_values: Vec<_> = str_array.iter().map(|s| s.map(|s_val| s_val)).flatten().collect();
                        Series::new(field.name(), &str_values)
                    }
                    _ => unimplemented!("Unsupported data type:{}", field.data_type()),
                }
            })
            .collect();
        let df = DataFrame::new(columns).unwrap();
        batches.push(df);
    }

    let mut df = batches[0].clone();
    for df_tmp in &batches[1..] {
        df.vstack_mut(&df_tmp);
    }

    let schema: Vec<DataFrameSchema> = df
        .get_columns()
        .iter()
        .map(|col| DataFrameSchema {
            name: col.name().to_string(),
            dtype: format!("{:?}", col.dtype()),
        })
        .collect();

    let mut data: Vec<DataFrameRow3> = Vec::new();

    for i in 0..df.height() {
        let mut row = serde_json::Map::new();
        row.insert("id".to_string(), i.into());
        for col in df.get_columns() {
            let value = col.get(i).unwrap();
            let json_value = match value {
                //AnyValue::Utf8(v) => serde_json::json!(v),
                AnyValue::Int32(v) => serde_json::json!(v),
                AnyValue::Float64(v) => serde_json::json!(v),
                AnyValue::Boolean(v) => serde_json::json!(v),
                AnyValue::Int64(v) => serde_json::json!(v),
                AnyValue::UInt32(v) => serde_json::json!(v),
                AnyValue::UInt64(v) => serde_json::json!(v),
                AnyValue::Float32(v) => serde_json::json!(v),
                //AnyValue::Utf8Owned(v) => serde_json::json!(v),
                //AnyValue::List(v) => serde_json::json!(v),
                AnyValue::Date(v) => serde_json::json!(v),
                AnyValue::Datetime(v, _, _) => serde_json::json!(v),
                AnyValue::Time(v) => serde_json::json!(v),
                AnyValue::Null => serde_json::json!(null),
                _ => serde_json::json!(format!("{:?}", value)), // デフォルトとしてデバッグ表記を使用
            };
            row.insert(col.name().to_string(), json_value);
        }
        data.push(DataFrameRow3 { row });
    }

    let response = DataFrameResponse { schema, data };
    serde_json::to_string(&response).map_err(|e| e.to_string())
}

#[command]
fn get_dataframe_dynamic(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(file_path.clone());
    println!("Received file path: {:?}", path);

    // HDF5 を読み込む
    let file = File::open(file_path.clone()).unwrap();
    let dataset = file.dataset("arrow_data").unwrap();

    get_dataframe_dynamic_local(&dataset)

    // // データを読み込む
    // let data: Vec<u8> = dataset.read_raw().unwrap();

    // // Arrow IPC 形式のデータを読み込む
    // let cursor = Cursor::new(data);
    // let reader = FileReader::try_new(cursor, None).unwrap();

    // // Arrow のレコードバッチを Polars の DataFrame に変換
    // let mut batches = Vec::new();
    // for batch in reader {
    //     let record_batch = batch.unwrap();

    //     let columns: Vec<Series> = record_batch
    //         .columns()
    //         .iter()
    //         .zip(record_batch.schema().fields())
    //         .map(|(array, field)| {
    //             match field.data_type() {
    //                 arrow::datatypes::DataType::Int64 => {
    //                     let int_array = array
    //                         .as_any()
    //                         .downcast_ref::<Int64Array>()
    //                         .expect("Failed to downcast to Int64Array");
    //                     Series::new(field.name(), int_array.values())
    //                 },
    //                 arrow::datatypes::DataType::Utf8 => {
    //                     let str_array = array
    //                         .as_any()
    //                         .downcast_ref::<StringArray>()
    //                         .expect("Failed to downcast to Utf8Array");
    //                     let str_values: Vec<_> = str_array.iter().map(|s| s.map(|s_val| s_val)).flatten().collect();
    //                     Series::new(field.name(), &str_values)
    //                 }
    //                 _ => unimplemented!("Unsupported data type:{}", field.data_type()),
    //             }
    //         })
    //         .collect();
    //     let df = DataFrame::new(columns).unwrap();
    //     batches.push(df);
    // }

    // let mut df = batches[0].clone();
    // for df_tmp in &batches[1..] {
    //     df.vstack_mut(&df_tmp);
    // }

    // let schema: Vec<DataFrameSchema> = df
    //     .get_columns()
    //     .iter()
    //     .map(|col| DataFrameSchema {
    //         name: col.name().to_string(),
    //         dtype: format!("{:?}", col.dtype()),
    //     })
    //     .collect();

    // let mut data: Vec<DataFrameRow3> = Vec::new();

    // for i in 0..df.height() {
    //     let mut row = serde_json::Map::new();
    //     row.insert("id".to_string(), i.into());
    //     for col in df.get_columns() {
    //         let value = col.get(i).unwrap();
    //         let json_value = match value {
    //             //AnyValue::Utf8(v) => serde_json::json!(v),
    //             AnyValue::Int32(v) => serde_json::json!(v),
    //             AnyValue::Float64(v) => serde_json::json!(v),
    //             AnyValue::Boolean(v) => serde_json::json!(v),
    //             AnyValue::Int64(v) => serde_json::json!(v),
    //             AnyValue::UInt32(v) => serde_json::json!(v),
    //             AnyValue::UInt64(v) => serde_json::json!(v),
    //             AnyValue::Float32(v) => serde_json::json!(v),
    //             //AnyValue::Utf8Owned(v) => serde_json::json!(v),
    //             //AnyValue::List(v) => serde_json::json!(v),
    //             AnyValue::Date(v) => serde_json::json!(v),
    //             AnyValue::Datetime(v, _, _) => serde_json::json!(v),
    //             AnyValue::Time(v) => serde_json::json!(v),
    //             AnyValue::Null => serde_json::json!(null),
    //             _ => serde_json::json!(format!("{:?}", value)), // デフォルトとしてデバッグ表記を使用
    //         };
    //         row.insert(col.name().to_string(), json_value);
    //     }
    //     data.push(DataFrameRow3 { row });
    // }

    // let response = DataFrameResponse { schema, data };
    // serde_json::to_string(&response).map_err(|e| e.to_string())
}

// --------------------------------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct Hdf5Node {
    name: String,
    full_key: String,
    children: Vec<Hdf5Node>,
}

fn read_hdf5_keys(file_path: &str) -> Result<Hdf5Node, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let root = file.group("/")?;
    let root_node = Hdf5Node {
        name: "/".to_string(),
        full_key: "/".to_string(),
        children: read_group(&root, &"".to_string())?,
    };
    Ok(root_node)
}

fn read_group(group: &hdf5::Group, parent_key: &String) -> Result<Vec<Hdf5Node>, Box<dyn std::error::Error>> {
    //println!("read_group: parent_key:{}", parent_key);
    let mut nodes = Vec::new();
    for member_name in group.member_names()? {
        let obj = group.group(&member_name);
        let base_key = format!("{}/{}", parent_key.clone(), member_name.clone());
        println!("read_group: base_key:{}", base_key);
        if let Ok(subgroup) = obj {
            nodes.push(Hdf5Node {
                name: member_name.clone(),
                full_key: base_key.clone(),
                children: read_group(&subgroup, &base_key)?,
            });
        } else {
            nodes.push(Hdf5Node {
                name: member_name.clone(),
                full_key: base_key.clone(),
                children: vec![],
            });
        }
    }
    Ok(nodes)
}


#[command]
fn get_hdf5_keys(file_path: String) -> Result<String, String> {
    println!("get_hdf5_keys: file path: {:?}", file_path);
    match read_hdf5_keys(&file_path) {
        Ok(tree) => serde_json::to_string(&tree).map_err(|e| e.to_string()),
        Err(e) => {
            println!("get_hdf5_keys: ERR: {:?}", e.to_string());
            Err(e.to_string())
        },
    }
}

// --------------------------------------------------------------------------------------------------------------------
fn convert_string(dataset: &Dataset) -> Result<String, Box<dyn std::error::Error>> {
    println!("convert_string : {:?}, ndim:{:?}", dataset.name(), dataset.ndim());

    let dtype = dataset.dtype()?;
    let shape = dataset.shape();
    println!("convert_string : dtype:{:?}, shape:{:?}", dtype.id(), shape);


    match dtype.to_descriptor() {
        Ok(type_descriptor) => {
            println!("convert_string : Ok(type_descriptor)");

            return match type_descriptor {
                TypeDescriptor::FixedArray(base, _size) => {
                    match *base {
                        TypeDescriptor::Integer(_size) => {
                            println!("convert_string : (FixedArray) TypeDescriptor::Integer(_size)");
                            let data = dataset.read_1d()?;
                            let data_vec: Vec<i64> = data.to_vec();
                            Ok(data_vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
                        },
                        TypeDescriptor::Unsigned(_size) => {
                            println!("convert_string : (FixedArray) TypeDescriptor::Unsigned(_size)");
                            let data = dataset.read_1d()?;
                            let data_vec: Vec<u64> = data.to_vec();
                            Ok(data_vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
                        },
                        TypeDescriptor::Float(_size) => {
                            println!("convert_string : (FixedArray) TypeDescriptor::Float(_size)");
                            let data = dataset.read_1d()?;
                            let data_vec: Vec<f64> = data.to_vec();
                            Ok(data_vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
                        },
                        _ => {
                            println!("convert_string - (FixedArray) - ERR: {:?}", base.to_string());
                            return Err("Unsupported data type".into())
                        },
                    }
                },
                TypeDescriptor::Integer(_size) => {
                    let data: i64 = dataset.read_scalar()?;
                    Ok(data.to_string())
                },
                TypeDescriptor::Unsigned(_size) => {
                    println!("convert_string : TypeDescriptor::Unsigned(_size)");
                    let data: u64 = dataset.read_scalar()?;
                    Ok(data.to_string())
                },
                TypeDescriptor::Float(_size) => {
                    println!("convert_string : TypeDescriptor::Float(_size)");
                    let data: f64 = dataset.read_scalar()?;
                    Ok(data.to_string())
                },
                TypeDescriptor::VarLenUnicode => {
                    println!("convert_string : TypeDescriptor::VarLenUnicode");
                    let data: VarLenUnicode = dataset.read_scalar()?;
                    Ok(data.as_str().to_string())

                    // let string_vec: Array1<VarLenUnicode> = dataset.read_1d()?;
                    
                    // let mut result_string = String::from("");
                    // for s in string_vec {
                    //     result_string.push_str(s.as_str());
                    // }
                    // Ok(result_string)
                },
                _ =>  {
                    println!("convert_string - FixedArray Not - ERR: {:?}", type_descriptor.to_string());
                    return Err("Unsupported data type".into())
                },
            }
        },
        Err(err) => {
            println!("convert_string - ERR: {:?}", err.to_string());
            return Err("Unsupported data type".into())
        },
    };
}

fn read_hdf5_data_local(file_path: String, full_key: String) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let dataset = file.dataset(&full_key);

    match dataset {
        Ok(ds) => {
            println!("read_hdf5_data_local : {:?}, ndim:{:?}", ds.name(), ds.ndim());
            let result = convert_string(&ds);
            match result {
                Ok(result_string) => Ok(result_string),
                Err(_) => {
                    Ok(get_dataframe_dynamic_local(&ds)?)
                }
            }
        },
        Err(err) => {
            let group = file.group(&full_key)?;
            Ok("".to_string())
        }
    }
}


#[command]
fn read_hdf5_data(file_path: String, full_key: String) -> Result<String, String> {
    
    println!("read_hdf5_data: file path: {:?}", file_path);
    println!("read_hdf5_data: full_key : {:?}", full_key);

    match read_hdf5_data_local(file_path, full_key) {
        Ok(result) => Ok(result),
        Err(e) =>  {
            println!("read_hdf5_data: ERR: {:?}", e.to_string());
            Err(e.to_string())
        },
    }
}


// --------------------------------------------------------------------------------------------------------------------
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_dataframe, 
            read_data_frame, 
            get_dataframe_dynamic, 
            get_hdf5_keys, 
            read_hdf5_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
