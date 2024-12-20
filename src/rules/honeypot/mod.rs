use std::{collections::HashMap, net::SocketAddr};


use serde::{Serialize, Deserialize}; // For the Serialize and Deserialize traits
//use serde_json; // For working with JSON, if required

//use serde_json::Value;
use tokio;

use crate::csv::header;
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
    client_addr: &SocketAddr,
    user_agents:Vec<header>
) -> Result<(), Box<dyn std::error::Error>> {
    let key_words = ["username", "password", "cgi", "root", "admin", "cd", "wp"];

    if let Some(agent) = request.get("User-Agent") {
        for h in user_agents {
            if agent.contains(&h.get_header()) {
                println!("Malcious use agent spotted when conmpared against csv")

                let query_agent = query(r#"
        MERGE (ip:IPAddress {address: $ip_address})
        MERGE (req:RequestString {content: $req_string})
        MERGE (ua:User-Agent {agent:$agent, description:$desc, tool:$tool, cat:$cat, link:$link, severity:$severity, meta_usage:&usage})
        MERGE (ip)-[:REQUESTED]->(req)
        MERGE (ip)-[:USED_USER_AGENT]->(ua)
        MERGE (ua)-[:MADE_REQUEST]->(req)
        "#).param("ip_address", client_addr.to_string().split_once(":").unwrap().0.to_string()).param("req_string", request.get("Received").unwrap().to_string())
        .param("agent", h.get_header())
        .param("desc", h.get_reason())
        .param("tool", h.get_tool())
        .param("cat", h.get_cat())
        .param("link", h.get_link())
        .param("severity", h.get_severity())
        .param("usage", h.get_meta());
            }
        }
    }
   

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

    
    
