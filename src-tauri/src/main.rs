#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use datafusion::{
    arrow::json::writer::record_batches_to_json_rows,
    prelude::{ParquetReadOptions, SessionContext, AvroReadOptions, SessionConfig, CsvReadOptions},
};
use tauri::{Result};

fn main() {
    tauri::Builder::default()
        .manage(FileState {})
        .invoke_handler(tauri::generate_handler![execute_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command]
// fn fileinput(filename: String, _state: tauri::State<FileState>) {
//     // If something fails
//     println!("file name is {} ", filename);
//     let f = File::open(filename).unwrap();
//     let is_file = f.metadata().unwrap().is_file();
//     println!("{}", is_file);
// }

#[tauri::command]
async fn execute_query(
    query: String,
    file: String,
    filetype: String
) -> Result<Vec<serde_json::Map<std::string::String, serde_json::Value>>> {
    println!("Query is {} ", query);
    let session_config = SessionConfig::default();
    let ctx = SessionContext::with_config(session_config.with_information_schema(true));
    let mut reg=Ok(());
    if filetype == "parquet" && query.contains("sample") {
      reg = ctx.register_parquet("sample", &file, ParquetReadOptions::default())
          .await;
    }else if filetype == "csv" && query.contains("sample"){
      reg = ctx.register_csv("sample", &file, CsvReadOptions::default())
          .await;
    }else if filetype == "json" && query.contains("sample"){
      reg = ctx.register_json("sample", &file, datafusion::prelude::NdJsonReadOptions::default())
          .await;
    } else if filetype == "avro" && query.contains("sample"){
      reg = ctx.register_avro("sample", &file, AvroReadOptions::default())
      .await;
    }
    match reg {
      Ok(()) => {},
      Err(e) => return Err(tauri::Error::AssetNotFound(e.to_string())),
    }
    let df_result = ctx.sql(&query).await;
    let mut df = match df_result {
        Ok(df1) => df1,
        Err(e) => return Err(tauri::Error::AssetNotFound(e.to_string())),
    };
    if !&query.contains("limit") {
      println!("query not containing limit");
      df = match  df.limit(Some(0), Some(100)){
        Ok(d) => d,
        Err(e) => return Err(tauri::Error::AssetNotFound(e.to_string())),
      };
    }
    ctx.deregister_table("sample").unwrap();
    match df.collect().await {
      Ok(df2) => return Ok(record_batches_to_json_rows(df2.as_slice()).unwrap()),
      Err(e) => return Err(tauri::Error::AssetNotFound(e.to_string())),
    };

    // Ok(df.unwrap())

    // let json = record_batches_to_json_rows(df.collect().await.unwrap().as_slice());
    // // let data = df.collect().await.unwrap();
    // Ok(json.unwrap())
}
struct FileState;
