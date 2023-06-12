use axum::{http::StatusCode, routing::post, Json, Router, Server};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{env, process::Command};

#[derive(Deserialize)]
struct GenerateCode {
    prompt: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/santacoder", post(santacoder_generate));
    Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn santacoder_generate(
    Json(payload): Json<GenerateCode>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Need to choose inference program and model"),
        ));
    }
    let inference_program_path = &args[1];
    let model_path = &args[2];
    let output = if let Ok(output) = Command::new(inference_program_path)
        .arg("-m")
        .arg(model_path)
        .arg("-p")
        .arg(payload.prompt)
        .output()
    {
        output
    } else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to run inference"),
        ));
    };
    return Ok(Json(json!({
        "output": String::from_utf8_lossy(&output.stdout).to_string(),
    })));
}
