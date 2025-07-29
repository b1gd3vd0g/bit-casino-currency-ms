mod db;
mod handlers;
mod requests;
mod router;

use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

use crate::{db::connect, router::router};

#[tokio::main]
async fn main() {
    let pool = connect().await;
    let app: Router = router().with_state(pool);

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on {}", address);
    axum::serve(listener, app).await.unwrap();
}
