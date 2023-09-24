use std::env;

use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, Error> {
    let path = event.path.unwrap();

    log::warn!("my_handler called: {}", path);
    
    let body = reqwest::get("https://www.etfscreen.com/muscular-portfolios/index.php?t=pd")
    .await?
    .text()
    .await?;
    log::warn!("body: {}", body);

    let zapier_webhook_url = env::var("ZAPIER_WEBHOOK_URL").unwrap();

    let client = reqwest::Client::new();
    let _res = client.post(zapier_webhook_url)
        .body(body)
        .send()
        .await;

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(format!("Hello from '{}'", path))),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}
