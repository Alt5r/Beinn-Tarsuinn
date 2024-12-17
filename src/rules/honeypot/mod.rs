use std::{collections::HashMap, os::unix::net::SocketAddr};

use reqwest;
use serde_json::Value;
//use tokio;

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use surrealdb::opt::auth::Namespace;
//use surrealdb::engine::any::Any;
use surrealdb::engine::remote::ws::Wss;





pub fn directoryChecker(request:HashMap<String, String>, client_addr:&SocketAddr) -> Result<(), Box<dyn std::error::Error>> {

    let key_words = ["username", "password", "cgi", "root", "admin", "cd"];
     // Connect to SurrealDB
    let mut db = Surreal::<Wss>::new("wss://testing-honeypo-069tap2kaptiv68a587151t91o.aws-euw1.surreal.cloud/rpc");
    //let db = Surreal::<Client>::connect("wss://testing-honeypo-069tap2kaptiv68a587151t91o.aws-euw1.surreal.cloud/rpc")?;

    db.signin(Namespace {
        namespace: "threat_monitoring", // Replace with your namespace
        username: "honeypot",      // Use 'honeypot' username as the 'email'
        password: "password",
    })?;
     // Specify namespace and database
    db.use_ns("testing").use_db("honeypot-data")?;

    if let Some(req) = request.get("Received") {
        let req = req.to_string();

        for k in key_words {
            if req.contains(k) {
                // malicious request, call database quesry for client_addr and for the malicious request
            }
        }
    }

        Ok(())
}