use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tauri::AppHandle;
use crate::core::api::service::{TokenService, ConfigService};

pub async fn start_server(handle: AppHandle) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 35555));
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[Token Server] Failed to bind to 127.0.0.1:35555: {}", e);
            return;
        }
    };

    println!("[Token Server] Listening on http://127.0.0.1:35555");

    loop {
        if let Ok((mut socket, _)) = listener.accept().await {
            let handle_clone = handle.clone();
            tokio::spawn(async move {
                handle_client(&mut socket, handle_clone).await;
            });
        }
    }
}

async fn handle_client(socket: &mut tokio::net::TcpStream, handle: AppHandle) {
    let mut buffer = [0; 1024];
    let n = match socket.read(&mut buffer).await {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let request = String::from_utf8_lossy(&buffer[..n]);
    let path = request.lines().next().unwrap_or("").split_whitespace().nth(1).unwrap_or("");

    match path {
        "/token" => handle_get_token(socket).await,
        "/config" => handle_get_config(socket, &handle).await,
        _ => send_error(socket, 404, "Not Found").await,
    }
}

async fn handle_get_token(socket: &mut tokio::net::TcpStream) {
    match TokenService::get_encrypted_token().await {
        Ok(res) => send_json(socket, 200, &res).await,
        Err(e) => send_error(socket, 500, &e).await,
    }
}

async fn handle_get_config(socket: &mut tokio::net::TcpStream, handle: &AppHandle) {
    match ConfigService::get_config(handle) {
        Ok(res) => send_json(socket, 200, &res).await,
        Err(e) => send_error(socket, 500, &e).await,
    }
}

async fn send_json<T: serde::Serialize>(socket: &mut tokio::net::TcpStream, status: u16, data: &T) {
    let body = serde_json::to_string(data).unwrap_or_default();
    send_response(socket, status, "application/json", &body).await;
}

async fn send_error(socket: &mut tokio::net::TcpStream, status: u16, message: &str) {
    let body = serde_json::json!({ "error": message }).to_string();
    send_response(socket, status, "application/json", &body).await;
}

async fn send_response(socket: &mut tokio::net::TcpStream, status: u16, content_type: &str, body: &str) {
    let status_text = match status {
        200 => "OK",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status, status_text, content_type, body.len(), body
    );

    let _ = socket.write_all(response.as_bytes()).await;
    let _ = socket.flush().await;
}
