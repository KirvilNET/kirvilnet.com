use axum::{self, http::StatusCode, response::IntoResponse};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Web;

async fn fallback_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    
    // Check if the file has an extension (actual file)
    if path.contains('.') {
        if let Some(file) = Web::get(path) {
            let content_type = mime_guess::from_path(path)
                .first_or_octet_stream()
                .as_ref()
                .to_string();
            
            return (
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, content_type)],
                file.data,
            ).into_response();
        }
    }
    
    // All other routes -> index.html (Vue Router will handle routing)
    if let Some(file) = Web::get("index.html") {
        return (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "text/html")],
            file.data,
        ).into_response();
    }
    
    (StatusCode::NOT_FOUND, "Not found").into_response()
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
		.route("/api/status", axum::routing::get(axum::http::StatusCode::OK))
		.fallback(fallback_handler)
        //.nest_service("/assets", ServeDir::new("web/assets"))
		//.nest_service("/media", ServeDir::new("web/media"))
		;

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}