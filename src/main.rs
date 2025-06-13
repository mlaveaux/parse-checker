use clap::Parser;
use http_body_util::BodyExt;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(name = "parser")]
#[command(about = "A parser checker with various conversion options")]
struct Cli {
    /// Use mcrl22lps conversion
    #[arg(long)]
    mcrl22lps: String,

    /// Use lps2pbes conversion
    #[arg(long)]
    lps2pbes: String,

    /// Use mcrl22lps-old conversion
    #[arg(long)]
    mcrl22lps_old: String,

    /// Use lps2pbes-old conversion
    #[arg(long)]
    lps2pbes_old: String,

    /// Server port
    #[arg(long, default_value = "3000")]
    port: u16,
}

#[derive(Deserialize)]
struct CheckRequest {
    text: String,
}

#[derive(Serialize)]
struct CheckResponse {
    result: String,
    success: bool,
}

async fn handle_request(req: Request<IncomingBody>) -> Result<Response<String>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::POST, "/api/check_mcrl2") => {
            // Parse the request body
            let body_bytes = match req.into_body().collect().await {
                Ok(collected) => collected.to_bytes(),
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body("Failed to read request body".to_string())
                        .unwrap());
                }
            };

            let check_request: CheckRequest = match serde_json::from_slice(&body_bytes) {
                Ok(req) => req,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body("Invalid JSON in request body".to_string())
                        .unwrap());
                }
            };

            // Process the mCRL2 text (placeholder implementation)
            let result = "test".to_string();
            
            let response = CheckResponse {
                result: result.clone(),
                success: true,
            };

            let response_json = serde_json::to_string(&response).unwrap();
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .body(response_json)
                .unwrap())
        }
        (&hyper::Method::GET, "/") => {
            // Serve the HTML file
            let html_content = include_str!("../webpage/index.html");
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(html_content.to_string())
                .unwrap())
        }
        (&hyper::Method::OPTIONS, "/api/check_mcrl2") => {
            // Handle CORS preflight
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .body("".to_string())
                .unwrap())
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Not Found".to_string())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    // Check if all provided tool files exist
    let mut missing_tools = Vec::new();

    if !cli.mcrl22lps.is_empty() && !std::path::Path::new(&cli.mcrl22lps).exists() {
        missing_tools.push(format!("mcrl22lps: {}", cli.mcrl22lps));
    }

    if !cli.lps2pbes.is_empty() && !std::path::Path::new(&cli.lps2pbes).exists() {
        missing_tools.push(format!("lps2pbes: {}", cli.lps2pbes));
    }

    if !cli.mcrl22lps_old.is_empty() && !std::path::Path::new(&cli.mcrl22lps_old).exists() {
        missing_tools.push(format!("mcrl22lps-old: {}", cli.mcrl22lps_old));
    }

    if !cli.lps2pbes_old.is_empty() && !std::path::Path::new(&cli.lps2pbes_old).exists() {
        missing_tools.push(format!("lps2pbes-old: {}", cli.lps2pbes_old));
    }

    if !missing_tools.is_empty() {
        eprintln!("Error: The following tool files do not exist:");
        for tool in missing_tools {
            eprintln!("  {}", tool);
        }
        std::process::exit(1);
    }

    // Run as HTTP server
    let addr = SocketAddr::from(([127, 0, 0, 1], cli.port));
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
