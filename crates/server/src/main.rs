use clap::Parser;
use http_body_util::BodyExt;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::process::ExitCode;
use tokio::net::TcpListener;
use duct::cmd;

#[derive(Clone)]
struct ToolConfig {
    mcrl22lps: String,
    lps2pbes: String,
    mcrl22lps_old: String,
    lps2pbes_old: String,
}

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

async fn handle_request(
    req: Request<IncomingBody>,
    config: ToolConfig,
) -> Result<Response<String>, Infallible> {
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

            // Run mcrl22lps --print-ast with the input text
            let output = cmd!(config.mcrl22lps, "--print-ast")
                .stdin_bytes(check_request.text.as_bytes())
                .stderr_to_stdout()
                .run();

            let (result, success) = match output {
                Ok(output) => {
                    if output.status.success() {
                        (String::from_utf8_lossy(&output.stdout).to_string(), true)
                    } else {
                        (String::from_utf8_lossy(&output.stdout).to_string(), false)
                    }
                },
                Err(e) => (format!("Error running mcrl22lps: {}", e), false),
            };
            
            let response = CheckResponse {
                result,
                success,
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
async fn main() -> Result<ExitCode, Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    // Check if all provided tool files exist
    let mut missing_tools = Vec::new();

    if !cli.mcrl22lps.is_empty() && !std::path::Path::new(&cli.mcrl22lps).exists() {
        eprintln!("mcrl22lps tool does not exist at: {}", cli.mcrl22lps);
    }

    if !cli.lps2pbes.is_empty() && !std::path::Path::new(&cli.lps2pbes).exists() {
        eprintln!("lps2pbes tool does not exist at: {}", cli.mcrl22lps);
    }

    if !cli.mcrl22lps_old.is_empty() && !std::path::Path::new(&cli.mcrl22lps_old).exists() {
        eprintln!("mcrl22lps tool (old) does not exist at: {}", cli.mcrl22lps);
    }

    if !cli.lps2pbes_old.is_empty() && !std::path::Path::new(&cli.lps2pbes_old).exists() {
        eprintln!("lps2pbes tool (old) does not exist at: {}", cli.mcrl22lps);
        missing_tools.push(format!("lps2pbes-old: {}", cli.lps2pbes_old));
    }

    // Create tool configuration
    let config = ToolConfig {
        mcrl22lps: cli.mcrl22lps.clone(),
        lps2pbes: cli.lps2pbes.clone(),
        mcrl22lps_old: cli.mcrl22lps_old.clone(),
        lps2pbes_old: cli.lps2pbes_old.clone(),
    };

    // Run as HTTP server
    let addr = SocketAddr::from(([127, 0, 0, 1], cli.port));
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let config_clone = config.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |req| handle_request(req, config_clone.clone()));
            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
