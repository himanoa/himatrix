use client_server_api::routes::*;
use client_server_api::state::State;
use envconfig::Envconfig;
use tide_tracing::TraceMiddleware;
use tracing::Level;
use tide::{Middleware, Request, Next, Server};

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "HOST", default = "127.0.0.1:8080")]
    pub host: String,
}

#[derive(Default)]
struct EnhanceCORSHeaderMiddleware {}

impl EnhanceCORSHeaderMiddleware {
    pub fn new() -> EnhanceCORSHeaderMiddleware {
        EnhanceCORSHeaderMiddleware {}
    }
}

#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> Middleware<T> for EnhanceCORSHeaderMiddleware {
    async fn handle(&self, req: Request<T>, next: Next<'_, T>) -> tide::Result {
        let mut res = next.run(req).await;
        res.insert_header("Access-Control-Allow-Origin", "*");
        res.insert_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
        res.insert_header("Access-Control-Allow-Headers", "X-Requested-With, Content-Type, Accept, Authorization");
        Ok(res)
    }
}

fn app(_config: &Config) -> Server<State> {
    let mut app = tide::Server::with_state(State::new());
    app.with(TraceMiddleware::new());
    app.with(EnhanceCORSHeaderMiddleware::new());

    app.at("/_matrix/client/versions")
        .get(versions::versions_endpoint);
    return app;
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config = Config::init_from_env().unwrap();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    app(&config).listen(config.host).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tide_testing::TideTestingExt;

    #[async_std::test]
    async fn test_feature_set_access_control_allow_origin() -> tide::Result<()> {
        let app = app(&Config { host: "".to_string() }); 
        assert_eq!(app.get("/_matrix/client/versions").await?.header("Access-Control-Allow-Origin").unwrap(), "*");
        Ok(())
    }

    #[async_std::test]
    async fn test_feature_set_access_control_allow_headers() -> tide::Result<()> {
        let app = app(&Config { host: "".to_string() }); 
        assert_eq!(app.get("/_matrix/client/versions").await?.header("Access-Control-Allow-Methods").unwrap(), "GET, POST, PUT, DELETE, OPTIONS");
        Ok(())
    }

    #[async_std::test]
    async fn test_feature_set_access_control_allow_methods() -> tide::Result<()> {
        let app = app(&Config { host: "".to_string() }); 
        assert_eq!(app.get("/_matrix/client/versions").await?.header("Access-Control-Allow-Headers").unwrap(), "X-Requested-With, Content-Type, Accept, Authorization");
        Ok(())
    }
}
