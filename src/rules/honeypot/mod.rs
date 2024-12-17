use std::{collections::HashMap, net::SocketAddr};

use reqwest;
//use serde_json::Value;
//use tokio;


use surrealdb::engine::any;
use surrealdb::Surreal;
use surrealdb::opt::auth::Namespace;
//use std::collections::HashMap;
//use std::net::SocketAddr;

pub async fn directoryChecker(
    request: HashMap<String, String>,
    client_addr: &SocketAddr
) -> Result<(), Box<dyn std::error::Error>> {
    let key_words = ["username", "password", "cgi", "root", "admin", "cd"];

    // Connect to SurrealDB asynchronously
    let db = any::connect("wss://testing-honeypo-069tap2kaptiv68a587151t91o.aws-euw1.surreal.cloud/rpc").await?;
    //db.connect("wss://testing-honeypo-069tap2kaptiv68a587151t91o.aws-euw1.surreal.cloud/rpc").await?;

    // Sign in with async call
    db.signin(Namespace {
        namespace: "testing",
        username: "honeypot",
        password: "password",
    }).await?;

    // Select the namespace and database
    db.use_ns("testing").use_db("honeypot-data").await?;

    println!("Successfully connected and authenticated to SurrealDB!");

    // Check for malicious requests
    if let Some(req) = request.get("Received") {
        let req_string = req.to_string();

        for keyword in key_words {
            if req_string.contains(keyword) {
                println!("Malicious request detected from {:?}!", client_addr);
                // Log or handle the malicious request here
                break;
            }
        }
    }

    Ok(())
}
