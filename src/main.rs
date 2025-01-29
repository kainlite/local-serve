use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::Parser;
use mime_guess::from_path;
use std::sync::Arc;
use std::{fs, path::PathBuf};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to serve files from
    #[arg(short, long, default_value = ".")]
    dir: PathBuf,

    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

async fn generate_directory_listing(path: &PathBuf) -> (HeaderMap, String) {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Directory listing</title>
    <style>
        body { font-family: system-ui, sans-serif; max-width: 800px; margin: 0 auto; padding: 2rem; }
        ul { list-style-type: none; padding: 0; }
        li { margin: 0.5rem 0; }
        a { text-decoration: none; color: #2563eb; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h1>Directory listing</h1>"#,
    );

    // Add parent directory link if we're not at the root
    if path.parent().is_some() {
        html.push_str(r#"<p><a href="../">📁 ..</a></p>"#);
    }

    html.push_str("<ul>");

    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        // Sort entries: directories first, then files, both alphabetically
        entries.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        for entry in entries {
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            if path.is_dir() {
                html.push_str(&format!("<li>📁 <a href=\"{}/\">{}/</a></li>", name, name));
            } else {
                html.push_str(&format!("<li>📄 <a href=\"{}\">{}</a></li>", name, name));
            }
        }
    }

    html.push_str("</ul></body></html>");

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/html; charset=utf-8".parse().unwrap(),
    );

    (headers, html)
}

async fn handle_dir(base_path: Arc<PathBuf>) -> impl IntoResponse {
    generate_directory_listing(&base_path).await
}

async fn handle_path(
    Path(path): Path<String>,
    base_path: Arc<PathBuf>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let full_path = base_path.join(&path);

    if !full_path.exists() {
        return Err((StatusCode::NOT_FOUND, "File not found"));
    }

    if !full_path.starts_with(&*base_path) {
        return Err((StatusCode::FORBIDDEN, "Access denied"));
    }

    if full_path.is_dir() {
        Ok(generate_directory_listing(&full_path).await)
    } else {
        let content = match fs::read(&full_path) {
            Ok(content) => content,
            Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file")),
        };

        let file_name = full_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("download");

        let mime_type = from_path(&full_path).first_or_octet_stream().to_string();
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, mime_type.parse().unwrap());
        headers.insert(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_name)
                .parse()
                .unwrap(),
        );

        Ok((headers, String::from_utf8(content.to_vec()).unwrap()))
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing with debug level
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .init();

    // Parse command line arguments
    let args = Args::parse();
    let dir_display = args.dir.display().to_string();
    let base_path = Arc::new(args.dir);

    // Create the router with both directory listing and file handling
    let app = Router::new()
        .route("/", {
            let base_path = base_path.clone();
            get(move || handle_dir(base_path))
        })
        .route("/{*path}", {
            let base_path = base_path.clone();
            get(move |path| handle_path(path, base_path))
        })
        .layer(TraceLayer::new_for_http());

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], args.port));

    info!("🚀 File server starting on http://{}", addr);
    info!("📂 Serving files from: {}", dir_display);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
