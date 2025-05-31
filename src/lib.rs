pub mod errors;
pub mod handlers;
pub mod models; 
pub mod services;

// Re-export for tests
pub use handlers::classify::classify_image;