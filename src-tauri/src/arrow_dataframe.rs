use std::io::Cursor;

use tauri::command;

use hdf5::types::{TypeDescriptor, VarLenUnicode};
use hdf5::File;
use hdf5::Dataset;

use arrow::array::{Int64Array, StringArray};
use arrow::ipc::reader::FileReader;

use polars::prelude::*;

use serde::Serialize;

#[command]
pub fn read_hdf5_data(file_path: String, full_key: String) -> Result<String, String> {
    
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

#[derive(Serialize)]
struct DataFrameRow {
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
    data: Vec<DataFrameRow>,
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
                        let str_values: Vec<String> = str_array.iter().map(|s| s.map(|s_val| s_val.to_string())).flatten().collect();
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

    let mut data: Vec<DataFrameRow> = Vec::new();

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
                _ => serde_json::json!(format!("{}", value.to_string())), // デフォルトとしてデバッグ表記を使用
            };
            row.insert(col.name().to_string(), json_value);
        }
        data.push(DataFrameRow { row });
    }

    let response = DataFrameResponse { schema, data };
    serde_json::to_string(&response).map_err(|e| e.to_string())
}

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
        Err(_) => {
            file.group(&full_key)?;
            Ok("".to_string())
        }
    }
}
