use client_server_api::routes::*;
use envconfig::Envconfig;
use tide_tracing::TraceMiddleware;
use tracing::Level;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "HOST", default = "127.0.0.1:8080")]
    pub host: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    let config = Config::init_from_env().unwrap();
    let mut app = tide::Server::new();
    app.with(TraceMiddleware::new());

    app.at("/_matrix/client/versions")
        .get(versions::versions_endpoint);
    app.listen(config.host).await?;
    Ok(())
}
