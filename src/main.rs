use poem::{
    get, handler,
    http::{Method, Uri},
    listener::TcpListener,
    web::Path,
    Route, Server,
};
use tracing::{error, info, warn};
use tracing_core::LevelFilter;
use tracing_subscriber::{prelude::*, registry::LookupSpan, EnvFilter, Layer, Registry};

#[handler]
fn hello(Path(name): Path<String>, method: Method, uri: &Uri) -> String {
    info!(
        http_request.request_method = %method,
        http_request.request_url = %uri,
        user = name,
        route = "/hello/:name"
    );

    format!("hello: {}", name)
}

#[handler]
fn error(Path(name): Path<String>, method: Method, uri: &Uri) -> String {
    error!(
        http_request.request_method = %method,
        http_request.request_url = %uri,
        user = name,
    );
    format!("hello: {}", name)
}

#[handler]
fn warning(Path(name): Path<String>, method: Method, uri: &Uri) -> String {
    warn!(
        http_request.request_method = %method,
        http_request.request_url = %uri,
        user = name,
    );
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = Registry::default();
    let cfg = LogConfig::new();
    let subscriber = subscriber.with(cfg.layer()).with(
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );
    tracing::subscriber::set_global_default(subscriber).expect("Could not set up global logger");

    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/warning/:name", get(warning))
        .at("/error/:name", get(error));
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()?;
    Server::new(TcpListener::bind(("0.0.0.0", port)))
        .run(app)
        .await?;
    Ok(())
}

struct LogConfig {
    is_prod: bool,
}

impl LogConfig {
    pub fn new() -> Self {
        let k_service = std::env::var("K_SERVICE");
        Self {
            is_prod: k_service.is_ok(),
        }
    }

    pub fn layer<S>(&self) -> Box<dyn Layer<S> + Send + Sync + 'static>
    where
        S: tracing_core::Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        if self.is_prod {
            let stackdriver = tracing_stackdriver::layer();
            Box::new(stackdriver)
        } else {
            let stdout_log = tracing_subscriber::fmt::layer();
            Box::new(stdout_log)
        }
    }
}
