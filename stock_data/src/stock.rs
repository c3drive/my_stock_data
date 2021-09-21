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

#[tokio::main]
async fn main() -> Result<(), Error> {
    // チャート画像URL取得
    //get_stockchart_imgurl().await.unwrap_or_else(|err| eprintln!("{}", err));
    func().await?;
    //let func = handler_fn(func);
    //lambda_runtime::run(func).await?;


    Ok(())
}


//async fn func(event: Value, _: Context) -> Result<Value, Error> {
async fn func() -> Result<Value, Error> {

    // チャート画像URL取得
    let url = get_stockchart_imgurl().await?;

    // 取得したurlで画像dL（部品か）
    get_stockchart_img(&url).await?;
    // 保存
    //wget --secure-protocol=auto "https://stockcharts.com/c-sc/sc?s=%24NIKK&p=D&b=5&g=0&i=0&r=1631528503869" --user-agent="Mozilla/5.0"


    // S3へ格納
    Ok(json!({ "message": format!("Ok, {}!", url) }))
}

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
async fn get_stockchart_img(url: &String) -> Result<(), Error> {

    // SetUp
    let mut chart = GetStockChartsImgIF::new(url);
    //let values = vec![String::from("$NIKK"),];
    //chart.add_param(values);

    // Request
    chart.send_request().await?;

    // Result
    let bodys = chart.get_content();
    let bytes = &bodys["bytes"];
    file_write(bytes).await?;
    //println!("{}", body);

    Ok(())

}
async fn file_write(bytes: &Bytes) -> Result<(), Error> {
    // write
    //let string = "Hello, file io!";
    //let mut f = File::create("test.png").unwrap(); // open file, you can write to file.
    let mut out = File::create("./test.png")?;
    // "create" open as write only mode.
    //f.write_all(string.as_bytes()).unwrap(); // byte-only
    io::copy(&mut bytes.as_ref(), &mut out)?;

    // file is closed here.
    Ok(())
}



#[cfg(test)]
mod tests {
     //use super::*;

     #[test]
     fn it_works() {
        assert!(true, "always true");
    }
}
