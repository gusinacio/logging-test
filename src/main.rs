use poem::{get, handler, listener::TcpListener, web::Path,  Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Route::new().at("/hello/:name", get(hello));
    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse()?;
    Server::new(TcpListener::bind(("0.0.0.0", port)))
        .run(app)
        .await?;
    Ok(())
}
