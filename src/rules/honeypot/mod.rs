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

#[derive(serde::Serialize, serde::Deserialize)]
struct MaliciousSignature {
    request:String
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
    let key_words = ["username", "password", "cgi", "root", "admin", "cd", "wp"];

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
                let ca = client_addr.to_string();
                let ip_port = ca.split_once(":");
                let ip = ip_port.unwrap().0.to_string();
                //let ta: Option<ThreatActor> = db.create("threat-actor").await?;

                let created: Option<ThreatActor> = db
                .create("threat-actors")
                .content(ThreatActor {
                    ipv4:ip
                })
                .await?;

                let mal_req: Option<MaliciousSignature> = db
                .create("malicious-requests")
                .content(MaliciousSignature {
                    request:req_string
                })
                .await?;
                let edge_query = format!(r#"
                LET ta = (SELECT id FROM threat-actors WHERE ipv4 = "{}") 
                LET ms = (SELECT id FROM malicious-requests WHERE request = "{val}")
                CREATE (ta->ms) 
                "#, ip, val=req_string);
        
            // Execute the edge creation query (replace "some-ip" and "some-request" with actual values)
                db.query(edge_query).await?;
                
            
                    println!("Created edge between ThreatActor and MaliciousSignature");
                } else {
                    println!("unable to create edge");
                }
                return Ok(());
            }
        }
    }

    Ok(())
    }
