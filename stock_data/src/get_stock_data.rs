use async_trait::{async_trait};
use stock_data::{Interface, InterfaceTrait, get_links};
use std::collections::HashMap;

pub struct GetStock {}
impl GetStock {
    pub fn new() -> Interface {
        let mut intf = Interface::new("https://stockcharts.com/h-sc/ui");
        intf.add_param(GetStock::param("$NIKK"));
        return intf;
    }
    //fn new(q: &str, order: &str) -> HashMap<String, String> {
    //fn new(q: &str) -> HashMap<String, String> {
    fn param(q: &str) -> HashMap<&str, &str> {
        let mut params= HashMap::new();
        params.insert("q", q);
        //params.insert(String::from("order"), String::from(order));
        return params;
    }
}
// struct GetStockChartInterface {}
// //impl InterfaceResult for GetStockChartResult {
// impl InterfaceTrait<Interface, InterfaceResponse> for GetStockChartInterface {
//     fn add_param(&mut self, params: HashMap<&str, &str>) {
//         //self.url.push_str(Url::parse_with_params(&self.url_base, params).unwrap().as_str());
//         self.url = String::from(Url::parse_with_params(&self.url_base, params).unwrap());
//     }
// }

#[tokio::main]
async fn main() {

    // リクエスト情報
    let mut chart = Interface::new("https://stockcharts.com/h-sc/ui");
    // chart.add_param(Params::make("$NIKK"));
    // //assert_eq!("", chart.content());

    // //TODO sendと融合
    // chart.send_request().await;
    //assert_eq!("", chart.content());

    //TODO パース
    //chart.approve();
    //assert_eq!("I ate a salad for lunch today", chart.content());
    // if let Ok(chart_response) = chart.send().await {
    //     //println!("size: {:?}", chart_response);
    //     //println!("size: {:?}", chart_response.chart_response.get_data());
    //     println!("size: {:?}", chart_response.get_url());
    //     //let parse = GetStockChartResult::new(chart_response);
    // }else {
    //     println!("err");
    // };
}