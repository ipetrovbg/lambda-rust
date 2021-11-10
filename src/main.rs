mod models;

use aws_sdk_sns::Client;
use lambda_runtime::Context;
use lambda_runtime::Error;
use models::{Event, Output};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn handler(event: Event, context: Context) -> Result<Output, Error> {
    let shared_config = aws_config::load_from_env().await;

    let client = Client::new(&shared_config);

    client
        .publish()
        .topic_arn("arn:aws:sns:eu-central-1:348327224059:EmailTopic")
        .message(format!(
            "{} {} {}",
            event.first_name, event.second_name, context.request_id
        ))
        .send()
        .await?;

    Ok(Output {
        message: format!("{} {}.", event.first_name, event.second_name),
        request_id: context.request_id,
    })
}
