mod args;
mod partful_types;

use args::{Commands, PartfulCli};
use clap::Parser;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
//use log::{debug, error, info, log_enabled, Level};
use url::form_urlencoded;

#[tokio::main]
async fn main() {
    //let env_logger::init();
    let args = PartfulCli::parse();

    println!("{:?}", args);

    // insert api key  or read from .env file
    let api_key = "";

    match &args.entity_type {
        Commands::List(part) => {
            list_parts(&part.search_string, part.skip, part.limit, api_key).await;
        }
        Commands::Create(part) => {
            create_part(api_key);
        }
        Commands::Delete(part) => {}
        Commands::Get(part) => {}
    }
}

async fn list_parts(search_string: &String, skip: usize, limit: usize, api_key: &str) {
    println!("List Parts");
    // Create a new HTTP client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Prepare the request <insert api call or read from .env file>
    let uri = "";

    let mut uri_base = "<url>/v1/part";

    // let uri2 = format!(
    //     "https://<url>/v1/part?skip=0&limit=10 {} {}",
    //     str1, str2
    // );

    if search_string != "" {}

    let request = Request::builder()
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .body(Body::empty())
        .unwrap();

    // Send the request and await the response
    let response = client.request(request).await.unwrap();

    // Read the response body as a string
    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();

    // Print the response
    println!("Response: {}", body_str);
    //debug!("Response: {}", body_str);
}

fn create_part_data(apit_key: &str) {}

async fn create_part(api_key: &str) {
    // Create a new HTTP client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Prepare the request body
    let request_body = r#"{
        "part": {
            "number": "Wheel123",
            "name": "Steering Wheel",
            "description": "A wheel that you steer with",
            "notes": "Vegan friendly faux leather 🐮"
        },
        "shopping": {
            "price": 67,
            "availability": true,
            "priceOnRequest": false
        }
    }"#;

    // Prepare the request
    let uri = "https://api.example.com/part";

    let request = Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .body(Body::from(request_body))
        .unwrap();

    // Send the request and await the response
    let response = client.request(request).await.unwrap();

    // Read the response body as a string
    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();

    // Print the response
    println!("Response: {}", body_str);
}

async fn delete_part(api_key: &str) {
    // Create a new HTTP client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Prepare the request
    let part_number = "Wheel123"; // Replace with the actual part number to delete
    let uri = format!("https://api.example.com/part/{}", part_number);
    let request = Request::builder()
        .method("DELETE")
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .body(Body::empty())
        .unwrap();

    // Send the request and await the response
    let response = client.request(request).await.unwrap();

    // Read the response body as a string
    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();

    // Print the response
    println!("Response: {}", body_str);
}

async fn get_part(api_key: &str) {
    // Create a new HTTP client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Prepare the request
    let part_number = "Wheel123"; // Replace with the actual part number to retrieve
    let uri = format!("https://api.example.com/part/{}", part_number);
    let request = Request::builder()
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .body(Body::empty())
        .unwrap();

    // Send the request and await the response
    let response = client.request(request).await.unwrap();

    // Read the response body as a string
    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();

    // Print the response
    println!("Response: {}", body_str);
}
