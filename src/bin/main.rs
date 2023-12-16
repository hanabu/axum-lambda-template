use axum_lambda_template::*;

/// main() function for regular server environment or localhost
#[tokio::main]
async fn main() -> Result<(), Error> {
    let listen = "127.0.0.1:3000".parse()?;
    axum::Server::bind(&listen)
        .serve(app().await.into_make_service())
        .await?;

    Ok(())
}
