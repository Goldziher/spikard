//! Generated test application
//! This is a minimal Spikard app built from fixture data

use spikard::AppError;

pub use spikard_e2e_app::*;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let app = create_app()?;

    app.run().await
}
