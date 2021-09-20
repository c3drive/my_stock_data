//! 株価データを取得するためのLambda関数プロジェクトです。
use lambda_runtime::{Error};
use crate::interfaces::Interface;
mod interfaces;
use interfaces::get_stockcharts::GetStockChartsIF;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //let func = handler_fn(my_handler);
    //lambda_runtime::run(func).await?;
    my_handler().await.unwrap_or_else(|err| eprintln!("{}", err));
    //println!("{:?}", my_handler().await);
    Ok(())
}


async fn my_handler() -> Result<(), Box<dyn std::error::Error>> {

    // SetUp
    let mut chart = GetStockChartsIF::new();
    let values = vec![String::from("$NIKK"),];
    chart.add_param(values);

    // Request
    chart.send_request().await?;

    // Result
    let body = chart.get_content();
    let url = &body["url"];
    println!("{}", url);

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
