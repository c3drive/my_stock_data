//! 株価データを取得するためのLambda関数プロジェクトです。

mod interfaces;
mod config;
use crate::config::config::CONFIG;
use crate::interfaces::{Interface, InterfaceDirect};
//use crate::config::config::CONFIG;

use bytes::Bytes;
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::io;
use std::fs::File;
use interfaces::get_stockcharts::GetStockChartsIF;
use interfaces::get_stockchartsimg::GetStockChartsImgIF;
use interfaces::manage_s3::ManageS3IF;

#[derive(Deserialize)]
struct CustomEvent {
    ticker: String,
}

#[derive(Serialize)]
struct CustomOutput {
    result: String,
    img_url: String,
    file_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    //func().await?;

    Ok(())
}

async fn func(event: CustomEvent, _: Context) -> Result<CustomOutput, Error> {
//async fn func() -> Result<CustomOutput, Error> {

    // チャート画像URL取得
    let url = get_stockchart_imgurl(&event).await?;

    // 取得したurlで画像DL
    let filename = get_stockchart_img(&event, &url).await?;

    // S3へ格納
    s3_push(&event, &filename).await?;

    Ok(CustomOutput {
        result: String::from(format!("Ok, {}!", event.ticker)),
        img_url: String::from(format!("{}", url)),
        file_name: String::from(format!("{}", filename)),
    })
}

/// チャート画像のURL取得
async fn get_stockchart_imgurl(event: &CustomEvent) -> Result<String, Error> {

    // SetUp
    let mut chart = GetStockChartsIF::new();
    let values = vec![String::from(&event.ticker),];
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
async fn get_stockchart_img(event: &CustomEvent, url: &String) -> Result<String, Error> {

    // SetUp
    let mut chart = GetStockChartsImgIF::new(url);

    // Request
    chart.send_request().await?;

    // Result
    let bodys = chart.get_content();
    let bytes = &bodys["bytes"];

    // ファイル保存
    let filename = save_file(event, bytes)?;

    Ok(filename)

}


/// S3へアップロード
async fn s3_push(event: &CustomEvent, filename: &str) -> Result<(), Error> {
    // アップロードディレクトリ
    let s3_object = String::from("stock_data/") + &event.ticker + "/";
    
    // SetUp
    let s3 = ManageS3IF::new(
        None, // デフォルトを利用する
        CONFIG.aws_s3_bucket.to_string(),
        s3_object + &filename,
    ).await;
 
    // Request
    let filepath = stock_data::lambda_file_dir() + filename;
    s3.push(&filepath).await?;

    Ok(())
}


// ファイル保存し、ファイル名を返却
fn save_file(event: &CustomEvent, bytes: &Bytes) -> Result<String, Error> {
    // ファイル名生成
    let filename = String::from(&event.ticker) + "_" + &(stock_data::get_yyyymmdd()) + ".png";

    // ファイル格納ディレクトリパス＋ファイル名
    let filepath = stock_data::lambda_file_dir() + &filename;
    // write
    stock_data::write_file(&filepath, &bytes)?;

    // file is closed here.
    Ok(filename)
}


#[cfg(test)]
mod tests {
     use super::*;

     #[test]
     fn it_works() {
        assert!(true, "always true");
    }

    //async関数は#[test]では使用できない
    //#[test]
    #[tokio::test]
    async fn func_response() -> Result<(), Error> {
        stock_data::make_log("[INFO]", "my_handler_response", "start");
        let ticker = "$NIKK";
        let file_name = String::from(ticker) + "_" + &(stock_data::get_yyyymmdd()) + ".png";
        let event = CustomEvent{ticker: String::from(ticker),};

        // 実行
        let response = func(event, Context::default()).await?;

        // 結果
        let json = CustomOutput {
            result: String::from(format!("Ok, {}!", ticker)),
            img_url: String::from(format!("{}", "No Testable")),
            file_name: String::from(format!("{}", file_name)),
        };
        // アサーション
        assert_eq!(response.result, json.result);
        assert_eq!(response.file_name, json.file_name);

        Ok(())
    }
}