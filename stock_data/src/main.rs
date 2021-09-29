use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

// エントリポイント
// Lambda特有のものを集約させる
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(event: Value, _: Context) -> Result<Value, Error> {
    let first_name = event["firstName"].as_str().unwrap_or("world");
    println!("{:?}", event);

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}

#[cfg(test)]
mod tests {
    use super::*; // 外部モジュール内のテスト配下にあるコードを内部モジュールのスコープに持っていく

    //async関数は#[test]では使用できない
    //#[test]
    #[tokio::test]
    async fn my_handler_response() -> Result<(), Error> {
        let event = json!({
            "firstName": "AWS Lambda on Rust"
        });
        let json = json!({
            "message": "Hello, AWS Lambda on Rust!",
        });
        //let func = handler_fn(my_handler);
        //let response= lambda_runtime::run(func).await?;
        let response = my_handler(event, Context::default()).await?;
        assert_eq!(response, json);
        Ok(())
    }
}