mod models;

use aws_sdk_sns::Client;
use lambda_runtime::Context;
use lambda_runtime::Error;
use models::{Event, Output};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn handler(event: Event, context: Context) -> Result<Output, Error> {
    let shared_config = aws_config::load_from_env().await;

    let client = Client::new(&shared_config);
    let topic_arn = env::var("TOPIC_ARN")?;

    client
        .publish()
        .topic_arn(topic_arn)
        .message(format!(
            "{} {}",
            event.message, context.request_id
        ))
        .send()
        .await?;

    Ok(Output {
        message: format!("{}", event.message),
        request_id: context.request_id,
    })
}
