use axum_lambda_template::*;

/// main() function for regular server environment or localhost
#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app().await).await?;

    Ok(())
}
