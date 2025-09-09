use clap::Parser;
use mongodb::{Client, options::ClientOptions};
use std::process;

#[derive(Parser)]
#[command(name = "mongodb-connection-test")]
#[command(about = "A simple MongoDB connection tester")]
struct Args {
    #[arg(help = "MongoDB connection URI")]
    uri: String,
    
    #[arg(short, long, help = "Username for authentication")]
    username: Option<String>,
    
    #[arg(short, long, help = "Password for authentication")]
    password: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("Testing MongoDB connection...");
    println!("URI: {}", args.uri);
    
    if let Some(ref username) = args.username {
        println!("Username: {}", username);
    }
    
    match test_connection(&args).await {
        Ok(_) => {
            println!("✅ Connection successful!");
            println!("Successfully connected to MongoDB database.");
        }
        Err(e) => {
            println!("❌ Connection failed!");
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}

async fn test_connection(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut uri = args.uri.clone();
    
    if let (Some(username), Some(password)) = (&args.username, &args.password) {
        if !uri.contains("://") {
            return Err("Invalid URI format".into());
        }
        
        let protocol_end = uri.find("://").unwrap() + 3;
        let credentials = format!("{}:{}@", username, password);
        uri.insert_str(protocol_end, &credentials);
    }
    
    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;
    
    client.database("admin").run_command(mongodb::bson::doc! {"ping": 1}).await?;
    
    Ok(())
}
