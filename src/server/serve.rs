use std::net::SocketAddrV4;
use tokio::net::TcpListener;

use super::router;

pub async fn serve(address: SocketAddrV4) {
    let router = router();
    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
