use clap::Parser;
use mongodb::{bson::doc, options::ClientOptions, options::ResolverConfig, Client};
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

    #[arg(short, long, help = "Database name to test (defaults to 'admin')")]
    database: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    println!("Testing MongoDB connection...");
    println!("URI: {}", args.uri);

    if let Some(ref username) = args.username {
        println!("Username: {}", username);
    }

    let database_name = args.database.as_deref().unwrap_or("admin");
    println!("Database: {}", database_name);

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
    let mut uri = args.uri.clone();

    if let (Some(username), Some(password)) = (&args.username, &args.password) {
        if !uri.contains("://") {
            return Err("Invalid URI format".into());
        }

        let protocol_end = uri.find("://").unwrap() + 3;
        let credentials = format!("{}:{}@", username, password);
        uri.insert_str(protocol_end, &credentials);
    }

    // println!("Parsing MongoDB connection options");
    // let client_options = ClientOptions::parse(&uri).resolver_config(value).await?;
    println!("Trying to parse options with cloudflare DSN resolver");
    let parse_func = || async {
        match ClientOptions::parse(&uri)
            .resolver_config(ResolverConfig::cloudflare())
            .await
        {
            Err(e) => {
                println!("DNS resolution failed with Cloudflare DNS: `{}`. Falling back to default hickory-resolver.", e);
                println!("Creating client options with default hickory-resolver");
                ClientOptions::parse(&uri).await
            }
            other => other,
        }
    };
    let client_options = parse_func().await?;

    println!("Creating MongoDB client with options");
    let client = Client::with_options(client_options)?;

    let database_name = args.database.as_deref().unwrap_or("admin");
    let database = client.database(database_name);

    println!("Testing connection with ping command");
    let ping_result = database.run_command(doc! {"ping": 1}).await?;

    println!("Ping result: {:?}", ping_result);

    println!("Testing database access by listing collections");
    let _collections = database.list_collection_names().await?;
    println!("Successfully listed collections");

    Ok(())
}
