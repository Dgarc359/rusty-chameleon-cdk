use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt, Response};
use dryoc::{classic::crypto_sign::crypto_sign_verify_detached};
use dotenv::dotenv;

/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    // Extract some useful information from the request
    dotenv().ok();
    let signature = event.headers().get("X-Signature-Ed25519").unwrap().to_owned();
    let sig_bytes: [u8; 64] =  signature.as_bytes().try_into().unwrap();

    let timestamp = event.headers().get("X-Signature-Timestamp").unwrap().to_string();
    let body = event.body();

    let message = timestamp.push_str(body).as_bytes();

    let public_key = dotenv::var("PUBLIC_KEY").unwrap();
    let pub_key_bytes: [u8; 32] = public_key.as_bytes().try_into().unwrap();

    // let token = env token
    let is_verified = crypto_sign_verify_detached(
        &sig_bytes,
        &message,
        &pub_key_bytes,
    );

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request")
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
