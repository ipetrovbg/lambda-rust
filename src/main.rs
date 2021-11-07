use lambda_runtime::Context;
use lambda_runtime::Error;
use serde::{Serialize, Deserialize};

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
#[derive(Serialize)]
struct Output {
    message: String,
    request_id: String,
}

#[derive(Deserialize)]
struct Event {
    first_name: String,
    second_name: String,
}