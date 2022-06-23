use lambda_http::{run, service_fn, Error, IntoResponse, Request, Response};
use std::env;
use std::fmt::Write;
// use sodiumoxide::crypto::sign;
use dryoc::classic;

use hex;
// use sodiumoxide::crypto::sign::ed25519::Signature::from_str(hex: &str);

/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {

    let signature = event.headers().get("X-Signature-Ed25519").unwrap();

    let mut sig_bytes = [0; 64];
    hex::decode_to_slice(signature, &mut sig_bytes)?;

    println!("{:?}", sig_bytes);

    let mut timestamp = String::from_utf8_lossy(event.headers().get("X-Signature-Timestamp").unwrap().as_bytes()).into_owned();

    write!(&mut timestamp, "{:?}", event.body()).unwrap();
    let message = timestamp.into_bytes();

    let public_key = env::var("PUBLIC_KEY").unwrap();
    let mut pub_key_bytes = [0; 32];
    
    hex::decode_to_slice(public_key, &mut pub_key_bytes)?;
    println!("{:?}",pub_key_bytes);
    
    Ok(match classic::crypto_sign::crypto_sign_verify_detached(
        &sig_bytes, 
        &message, 
        &pub_key_bytes) {
        Ok(_) => Response::builder().status(200).header("content-type", "text/html").body("'{ \"type\": 1 }'").map_err(Box::new)?,
        Err(_e) => Response::builder().status(401).body("Invalid request signature")?,
    })
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
