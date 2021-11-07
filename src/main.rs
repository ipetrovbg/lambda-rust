mod models;

use models::{Event, Output};
use lambda_runtime::Context;
use lambda_runtime::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn handler(event: Event, context: Context) -> Result<Output, Error> {
    Ok(Output {
        message: format!("{} {}.", event.first_name, event.second_name),
        request_id: context.request_id,
    })
}

