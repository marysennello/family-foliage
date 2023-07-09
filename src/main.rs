//! This is an example function that leverages the Lambda Rust runtime's HTTP support
//! and the [axum](https://docs.rs/axum/latest/axum/index.html) web framework.  The
//! runtime HTTP support is backed by the [tower::Service](https://docs.rs/tower-service/0.3.2/tower_service/trait.Service.html)
//! trait.  Axum applications are also backed by the `tower::Service` trait.  That means
//! that it is fairly easy to build an Axum application and pass the resulting `Service`
//! implementation to the Lambda runtime to run as a Lambda function.  By using Axum instead
//! of a basic `tower::Service` you get web framework niceties like routing, request component
//! extraction, validation, etc.

use aws_sdk_dynamodb::{Client, Error as OtherError};
use axum::{
    extract::Path,
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use serde::{Deserialize, Serialize};
use serde_dynamo::aws_sdk_dynamodb_0_28::from_items;

use serde_json::{json, Value};

const TABLE_NAME: &str = "family-foliage-people";

#[derive(Serialize, Deserialize)]
pub struct Person {
    id: u8,
    first_names: String,
    last_name: String,
    bio: String,
}

async fn get_tree(State(state): State<Client>) -> Json<Value> {
    Json(json!({ "msg": "I am GET /api/tree" }))
}

async fn get_tree_id(State(client): State<Client>, Path(id): Path<String>) -> Json<Value> {
    let results = client
        .query()
        .table_name(TABLE_NAME)
        .key_condition_expression(format!("#Id = :{id}"))
        .send()
        .await
        .unwrap();
    if let Some(items) = results.items {
        let people: Vec<Person> = from_items(items).unwrap();
        Json(json!(people))
    } else {
        Json(json!({}))
    }
}

async fn get_bio_id(State(state): State<Client>, Path(id): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am GET /foo/:id, id={id}") }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new()
        .route("/api/tree", get(get_tree))
        .route("/api/tree/:id", get(get_tree_id))
        .route("/api/bio/:id", get(get_bio_id))
        .with_state(client);

    run(app).await
}
