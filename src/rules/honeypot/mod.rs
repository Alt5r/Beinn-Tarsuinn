use std::{collections::HashMap, net::SocketAddr};

use reqwest;
use serde::{Serialize, Deserialize}; // For the Serialize and Deserialize traits
//use serde_json; // For working with JSON, if required

//use serde_json::Value;
//use tokio;
#[derive(serde::Serialize, serde::Deserialize)]
struct ThreatActor {
    ipv4:String,
}

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
                            // Create the ThreatActor instance
                /* 
                let threat_actor: ThreatActor = ThreatActor {
                    ipv4: client_addr.to_string(),
                };
                */
                let resc: &str = "threat-actors";
                // Log or handle the malicious request here
                //db.update::<ThreatActor>("threat-actors").merge(threat_actor).await?;

                //let ta: Option<ThreatActor> = db.create("threat-actor").await?;

                let created: Option<ThreatActor> = db
                .create("threat-actor")
                .content(ThreatActor {
                    ipv4:client_addr.to_string()
                })
                .await?;

            }
        }
    }

    Ok(())
}
