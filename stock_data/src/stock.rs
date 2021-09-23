//! 株価データを取得するためのLambda関数プロジェクトです。
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};
use bytes::Bytes;
use std::io;
use std::fs::File;
use crate::interfaces::{Interface, InterfaceDirect};
mod interfaces;
use interfaces::get_stockcharts::GetStockChartsIF;
use interfaces::get_stockchartsimg::GetStockChartsImgIF;
use interfaces::manage_s3::ManageS3IF;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    //func().await?;

    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
//async fn func() -> Result<Value, Error> {

    // チャート画像URL取得
    let url = get_stockchart_imgurl().await?;

    // 取得したurlで画像DL
    let filename = get_stockchart_img(&url).await?;

    // S3へ格納
    s3_push(&filename).await?;

    Ok(json!({ "message": format!("Ok, {}!", url) }))
}

/// チャート画像のURL取得
async fn get_stockchart_imgurl() -> Result<String, Error> {

    // SetUp
    let mut chart = GetStockChartsIF::new();
    let values = vec![String::from("$NIKK"),];
    chart.add_param(values);

    // Request
    chart.send_request().await?;

    // Result
    let bodys = chart.get_content();
    let url = &bodys["url"];
    println!("{}", url);

    Ok(String::from(url))
}

/// チャート画像のURLから画像取得しローカル 保存しファイル名返却
async fn get_stockchart_img(url: &String) -> Result<String, Error> {

    // SetUp
    let mut chart = GetStockChartsImgIF::new(url);

    // Request
    chart.send_request().await?;

    // Result
    let bodys = chart.get_content();
    let bytes = &bodys["bytes"];

    // ファイル保存
    let filepath = file_write(bytes).await?;

    Ok(filepath)

}


/// S3へアップロード
async fn s3_push(filename: &str) -> Result<(), Error> {
    
    // SetUp
    let s3 = ManageS3IF::new(
        None, // デフォルトを利用する
        String::from("my-work-project-bucket"),
        String::from(filename),
    ).await;
 
    // Request
    let filepath = lambda_file_dir() + filename;
    s3.push(&filepath).await?;

    Ok(())
}


/// Lambdaにおけるファイル格納場所（ここ以外保存しようとすると権限がなくエラーになる）
fn lambda_file_dir() -> String {
    return String::from("/tmp/");
}

// ファイル保存し、ファイル名を返却
async fn file_write(bytes: &Bytes) -> Result<String, Error> {
    stock_data::make_log("[INFO]", "file_write", "start");

    // ファイル名生成
    let filename = stock_data::get_yyyymmdd() + ".png";

    // ファイル格納ディレクトリパス＋ファイル名
    let filepath = lambda_file_dir() + &filename;
    // write
    let mut out = File::create(&filepath)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    // file is closed here.
    Ok(filename)
}


#[cfg(test)]
mod tests {
     //use super::*;

     #[test]
     fn it_works() {
        assert!(true, "always true");
    }
}
