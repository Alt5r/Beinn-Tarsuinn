use std::{collections::HashMap, net::SocketAddr};

use reqwest;
use serde::{Serialize, Deserialize}; // For the Serialize and Deserialize traits
//use serde_json; // For working with JSON, if required

//use serde_json::Value;
use tokio;
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

use neo4rs::{Graph, query};

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
            if req_string.contains(keyword) || true {
                println!("Malicious request detected from {:?}!", client_addr);
                            // Create the ThreatActor instance
                let ca = client_addr.to_string();
                let ip_port = ca.split_once(":");
                let ip = ip_port.unwrap().0.to_string();
                //let ta: Option<ThreatActor> = db.create("threat-actor").await?;
                /* 
                let created: Option<ThreatActor> = db
                .create("threat-actors")
                .content(ThreatActor {
                    ipv4:ip.clone()
                })
                .await?;

                let mal_req: Option<MaliciousSignature> = db
                .create("malicious-requests")
                .content(MaliciousSignature {
                    request:req_string.clone()
                })
                .await?;
            */
            // Query to get the ThreatActor ID

            /* 
            let ta_query = format!(r#"
            SELECT id FROM threat-actors WHERE ipv4 = "{}" LIMIT 1
            "#, ip);

            println!("Executing ThreatActor query: {}", ta_query);

            let ta_result = db.query(ta_query).await?;
            let ta_id: Option<String> = ta_result.first()
            .and_then(|res| res.get("id").and_then(|val| val.to_string().into()));

            if ta_id.is_none() {
            eprintln!("No ThreatActor found for IP: {}", ip);
            return Ok(()); // Skip further processing if no ThreatActor found
            }

            // Query to get the MaliciousSignature ID
            let ms_query = format!(r#"
            SELECT id FROM malicious-requests WHERE request = "{}" LIMIT 1
            "#, req_string);

            println!("Executing MaliciousSignature query: {}", ms_query);

            let ms_result = db.query(ms_query).await?;
            let ms_id: Option<String> = ms_result.first()
            .and_then(|res| res.get("id").and_then(|val| val.to_string().into()));

            if ms_id.is_none() {
            eprintln!("No MaliciousSignature found for request: {}", req_string);
            return Ok(()); // Skip further processing if no MaliciousSignature found
            }

            // Query to create the edge (RELATE)
            let relate_query = format!(r#"
            RELATE {}->malicious-request->{}
            "#, ta_id.unwrap(), ms_id.unwrap());

            println!("Executing RELATE query: {}", relate_query);

            let relate_result = db.query(relate_query).await;

            match relate_result {
            Ok(_) => println!("Successfully created edge between ThreatActor and MaliciousSignature."),
            Err(e) => eprintln!("Error creating edge: {:?}", e),
            }
            */


            // trying the neo4j solution


            let uri = "neo4j://192.168.1.128:7687".to_string();
            let user = "neo4j".to_string();
            let pass = "Password".to_string();

            let graph = Graph::new(&uri, user, pass).await?;

            let create_query = query(r#"
        MERGE (ip:IPAddress {address: $ip_address})
        MERGE (req:RequestString {content: $req_string})
        MERGE (ip)-[:REQUESTED]->(req)
        "#).param("ip_address", ip).param("req_string", req_string);

         graph.run(create_query).await?;



                } else {
                    println!("no amlicious content");
                }
                return Ok(());
            }
        }
        Ok(())
    }

    
    
