use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use serde_json::{json, Value};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let func = handler_fn(handler);
  lambda_runtime::run(func).await?;
  Ok(())
}

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
  Ok(json!({ "Hello": "World" }))
}