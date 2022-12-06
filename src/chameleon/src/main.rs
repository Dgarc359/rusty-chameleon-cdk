use dryoc::classic;
use lambda_http::{run, service_fn, Error, IntoResponse, Request, Response};
use std::env;
use serde::{Serialize, Deserialize};
use serde_json::{Value};

fn verify_key(
		body: &[u8],
		signature: &[u8],
		timestamp: &[u8],
		public_key: &[u8],
) -> Result<bool, Error> {
		let message = [timestamp, body].concat();
		let mut sig = [0u8; 64];
		let mut pk = [0u8; 32];

		hex::decode_to_slice(signature, &mut sig)?;
		hex::decode_to_slice(public_key, &mut pk)?;

		if classic::crypto_sign::crypto_sign_verify_detached(&sig, &message, &pk).is_ok() {
				Ok(true)
		} else {
				Ok(false)
		}
}

#[derive(Serialize, Deserialize, Debug)]
struct DiscordData {
		name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomBody {
		data: DiscordData,
		#[serde(rename = "type")]
		kind: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageData {
  url: string,
  height: i32,
  width: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedData {
  #[serde(rename = "type")]
  kind: String,
  title: String,
  color: i32,
  image: ImageData
}

struct EmbedResponseData {
  content: String,
  embeds: Vec<EmbedData>
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
	content: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct CustomResponse {
  #[serde(rename = "type")]
  kind: i64,
  data: ResponseData
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomEmbedResponse {
  #[serde(rename = "type")]
  kind: i64,
  data: EmbedResponseData
}

async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
		let signature = event
				.headers()
				.get("X-Signature-Ed25519")
				.unwrap()
				.to_str()?;

		let timestamp = event
				.headers()
				.get("X-Signature-Timestamp")
				.unwrap()
				.to_str()?;

		let public_key = env::var("PUBLIC_KEY")?;
		// dbg!("{#?}", event);

		Ok(
				match verify_key(
						event.body(),
						signature.as_bytes(),
						timestamp.as_bytes(),
						public_key.as_bytes(),
				) {
						Ok(ok) if ok => {
								println!("Building OK response");
								let body: CustomBody = serde_json::from_slice(&event.body() as &[u8]).unwrap();
								// let body: CustomBody = serde_json::from_str(&event.body() as String).unwrap()?;
								if &body.kind == &1i64 {
										println!("Received Ping for ack");
										Response::builder()
												.status(200)
												.header("content-type", "application/json")
												.body("{ \"type\": 1 }".to_string())
												.map_err(Box::new)?
								} else if &body.kind == &2i64 {
										println!("Application command received");
										let command_name: String = body.data.name;
										println!("command_name: {:?}", command_name);

										if command_name == "foo" {
												println!("command foo activated");
												let res = CustomResponse {
													kind: 4,
													data: ResponseData { content: "bar".to_owned() }
												};
												return Ok(Response::builder()
														.status(200)
														.header("content-type", "application/json")
														.body(serde_json::to_string(&res).unwrap())
														.map_err(Box::new)?);
										} else if command_name == "space" {
                      let res = CustomEmbedResponse {
                        kind: 4,
                        data: EmbedResponseData { content: "blahblah".to_owned(), embeds: vec![
                          EmbedData { kind: "rich".to_owned(), title: "Fake title".to_owned(), color: 0x00FFFF, image: ImageData { url: "https://apod.nasa.gov/apod/image/2212/M16Pillar_WebbOzsarac_1668.jpg".to_owned(), height: 0, width: 0 } }
                        ] 
                      };

                      return Ok(Response::builder()
														.status(200)
														.header("content-type", "application/json")
														.body(serde_json::to_string(&res).unwrap())
														.map_err(Box::new)?);

                      }
                    }

										// methods for different types of application commands
										// can probably leverage serenity constructs here
										// id	        snowflake	                                                the ID of the invoked command
										// name	        string	                                                    the name of the invoked command
										// type	        integer	                                                    the type of the invoked command
										// resolved?	resolved data	                                            converted users + roles + channels + attachments
										// options?*	array of application command interaction data option	    the params + values from the user
										// guild_id?	snowflake	                                                the id of the guild the command is registered to
										// target_id?	snowflake	                                                id of the user or message targeted by a user or message command

										Response::builder()
												.status(200)
												.header("content-type", "application/json")
												.body("{ \"type\": 1 }".to_string())
												.map_err(Box::new)?
								} else if &body.kind == &3i64 {
										println!("Message component received");
										Response::builder()
												.status(200)
												.header("content-type", "application/json")
												.body("{ \"type\": 1 }".to_string())
												.map_err(Box::new)?
								} else if &body.kind == &4i64 {
										println!("Application command autocomplete received");
										Response::builder()
												.status(200)
												.header("content-type", "application/json")
												.body("{ \"type\": 1 }".to_string())
												.map_err(Box::new)?
								} else if &body.kind == &5i64 {
										println!("Modal submit received");
										Response::builder()
												.status(200)
												.header("content-type", "application/json")
												.body("{ \"type\": 1 }".to_string())
												.map_err(Box::new)?
								} else {
										println!("Unknown interaction type received, sending default response");
										Response::builder()
												.status(200)
												.header("content-type", "application/json")
												.body("{ \"type\": 1 }".to_string())
												.map_err(Box::new)?
								}

						}
						Ok(_) => Response::builder()
								.status(401)
								.body("Invalid request signature".to_string())?,
						Err(e) => {
								println!("An unknown error occured {:?}", e);

								Response::builder()
										.status(500)
										.body(format!("An unknown error occured {:?}", e))?
						}
				},
		)
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
