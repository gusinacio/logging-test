use poem::{get, handler, listener::TcpListener, web::Path,  Route, Server, http::{Uri, Method}};
use tracing::{error, info, warn};
use tracing_subscriber::{Registry, prelude::*};

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
    let stackdriver = tracing_stackdriver::layer(); // writes to std::io::Stdout
    let subscriber = Registry::default().with(stackdriver);

    tracing::subscriber::set_global_default(subscriber).expect("Could not set up global logger");
    info!("{}", std::env::var("K_SERVICE").unwrap_or_else(|_| "Not wrapped".to_string()));


    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/warning/:name", get(warning))
        .at("/error/:name", get(error));
    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse()?;
    Server::new(TcpListener::bind(("0.0.0.0", port)))
        .run(app)
        .await?;
    Ok(())
}
