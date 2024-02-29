use axum::{routing::get, Router};
use wit_document::app_endpoint;

#[test]
fn ready() {
    println!("it works!")
}

#[tokio::test]
async fn preview() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    // build our application with a single route
    let app = Router::new().route("/", get(app_endpoint)).into_make_service();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
