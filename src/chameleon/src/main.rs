use lambda_http::{run, service_fn, Error, IntoResponse, Request, Response};
use dryoc::{classic::crypto_sign::crypto_sign_verify_detached};
// use reqwest;
use std::env;
use std::fmt::Write;

/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {

    let signature = event.headers().get("X-Signature-Ed25519").unwrap();
    let sig_bytes: [u8; 64] =  signature.as_bytes().try_into().unwrap();

    let mut timestamp = String::from_utf8_lossy(event.headers().get("X-Signature-Timestamp").unwrap().as_bytes()).into_owned();

    // let body = format!("{:?}", event.body());

    write!(&mut timestamp, "{:?}", event.body()).unwrap();
    let message = timestamp.into_bytes();

    let public_key = env::var("PUBLIC_KEY").unwrap();
    let pub_key_bytes: [u8; 32] = public_key.as_bytes().try_into().unwrap();

    // let token = env token
    Ok(match crypto_sign_verify_detached(
        &sig_bytes,
        &message,
        &pub_key_bytes,
    ) {
        Ok(_) => {
            // if interaction in request, then post interaction to discord's endpoint
            // let client = reqwest::Client::new();
            // let res = client.post("http://httpbin.org/post")
            //     .body("body")
            //     .send()
            //     .await();
            
            Response::builder().status(200).header("content-type", "text/html").body(";{ \"type\": 1 }'").map_err(Box::new)?
        },
        Err(_e) => Response::builder().status(401).body("Invalid request signature")?,
    })


    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    // let resp = Response::builder()
    //     .status(200)
    //     .header("content-type", "text/html")
    //     .body("'{ \"type\": 1 }'")
    //     .map_err(Box::new)?;
    
    // Ok(resp)
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
