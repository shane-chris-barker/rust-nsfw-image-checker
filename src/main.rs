use tokio::net::TcpListener;
use tracing_subscriber;
mod handlers;
mod errors;
mod models;
mod services;
use crate::errors::AppError;
use services::app::build_app;

#[tokio::main]
async fn main() -> Result<(), AppError>{
    tracing_subscriber::fmt::init(); 
    let app = build_app().await?;

    let listener = TcpListener::bind("0.0.0.0:3000").await
            .map_err(|e| AppError::Internal(format!("Failed to bind to port: {e}")))?;
    
    axum::serve(listener, app).await
            .map_err(|e| AppError::Internal(format!("Server error: {e}")))?;
    Ok(())
}