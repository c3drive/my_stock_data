//! 株価データを取得するためのLambda関数プロジェクトです。
//use std::fs;
//use std::io::{Write, Read, BufWriter, BufReader, copy};
use lambda_runtime::{Error};
//use lambda_runtime::{handler_fn, Context, Error};
// use reqwest::StatusCode;
//use serde_json::{json, Value};
use stock_data::{Interface, InterfaceTrait};
use std::collections::HashMap;

mod get_stock_data;

pub struct Params {}
impl Params {
    //fn new(q: &str, order: &str) -> HashMap<String, String> {
    fn new(q: &str) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("q"), String::from(q));
        //params.insert(String::from("order"), String::from(order));
        return params;
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    //let func = handler_fn(my_handler);
    //lambda_runtime::run(func).await?;
    my_handler().await;
    Ok(())
}


async fn my_handler() -> Result<(), Box<dyn std::error::Error>> {

    // リクエスト情報
    let mut chart = Interface::new("https://stockcharts.com/h-sc/ui");
    chart.add_param(Params::new("$NIKK"));
    assert_eq!("", chart.content());
    if let Ok(encoded) = chart.send().await {
        println!("size: {:?}", encoded.get_url());
    }else {
        println!("err");
    };
    // urlを取得 getBody()
    // 目的のurlを抽出する（これは共通化の必要ない？ここでやる）
    //let body = encoded.text().await?;
    //println!("response is \n{}", body);
    //file_write(body);

    //let links = get_links(body, "https:".to_string())?;
    //for link in links.iter() {
    //    println!("chart: {}", link);
    //}
    // 取得したurlで画像dL（部品か）
    // 保存
    //wget --secure-protocol=auto "https://stockcharts.com/c-sc/sc?s=%24NIKK&p=D&b=5&g=0&i=0&r=1631528503869" --user-agent="Mozilla/5.0"
    Ok(())
}


// fn file_write(string: String) {
//     {
//         // write
//         //let string = "Hello, file io!";
//         let mut f = fs::File::create("test.txt").unwrap(); // open file, you can write to file.
//         // "create" open as write only mode.
//         f.write_all(string.as_bytes()).unwrap(); // byte-only

//         // file is closed here.
//     }
// }



#[cfg(test)]
mod tests {
     //use super::*;

     #[test]
     fn it_works() {
        assert!(true, "always true");
    }
}
