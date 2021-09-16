use async_trait::{async_trait};
use lambda_http::Handler;
use stock_data::{Interface, InterfaceTrait, InterfaceResponse};
use std::collections::HashMap;

// pub struct GetStockChart {
//     state: Option<Box<(dyn State + 'static + Sync + Send)>>,
//     url_base: String,
//     param: String,
//     content: String,
// }

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

#[derive(Debug)]
pub struct GetStockChartResult {
    pub chart_response: InterfaceResponse,
    chart_url: String
}

//impl InterfaceResult for GetStockChartResult {
impl GetStockChartResult {
    // コンストラクタを提供することで構造体のフィールドはprivateのままとなる
    pub fn new(chart_response: InterfaceResponse) -> Self {
        GetStockChartResult {
            chart_response: chart_response,
            chart_url: "eeee".to_string(),
        }
    }
    
    fn on_parse(&self) -> String {
        "GetStockChart parse".to_string()
    }
    pub fn get_url(&self) -> &String {
        &self.chart_url
    }
}

#[tokio::main]
async fn main() {

    // リクエスト情報
    let mut chart = Interface::new("https://stockcharts.com/h-sc/ui");
    chart.add_param(Params::new("$NIKK"));
    assert_eq!("", chart.content());

    //TODO sendと融合
    chart.send_request();
    assert_eq!("", chart.content());

    //TODO パース
    chart.approve();
    assert_eq!("I ate a salad for lunch today", chart.content());
    if let Ok(chart_response) = chart.send().await {
        //println!("size: {:?}", chart_response);
        //println!("size: {:?}", chart_response.chart_response.get_data());
        println!("size: {:?}", chart_response.get_url());
        //let parse = GetStockChartResult::new(chart_response);
    }else {
        println!("err");
    };
}