use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tauri::AppHandle;
use crate::core::api::service::TokenService;

pub async fn start_server(_handle: AppHandle) {
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
        match listener.accept().await {
            Ok((mut socket, _)) => {
                tokio::spawn(async move {
                    let mut buffer = [0; 1024];
                    if let Ok(n) = socket.read(&mut buffer).await {
                        let request = String::from_utf8_lossy(&buffer[..n]);
                        
                        if request.starts_with("GET /token") {
                            handle_get_token(&mut socket).await;
                        } else {
                            handle_not_found(&mut socket).await;
                        }
                    }
                });
            }
            Err(e) => eprintln!("[Token Server] Connection failed: {}", e),
        }
    }
}

async fn handle_get_token(socket: &mut tokio::net::TcpStream) {
    match TokenService::get_encrypted_token().await {
        Ok(token_response) => {
            let body = serde_json::to_string(&token_response).unwrap_or_default();
            send_response(socket, 200, "application/json", &body).await;
        }
        Err(e) => {
            let error_json = serde_json::json!({ "error": e }).to_string();
            send_response(socket, 500, "application/json", &error_json).await;
        }
    }
}

async fn handle_not_found(socket: &mut tokio::net::TcpStream) {
    let body = "{\"error\": \"Not Found\"}";
    send_response(socket, 404, "application/json", body).await;
}

async fn send_response(socket: &mut tokio::net::TcpStream, status: u16, content_type: &str, body: &str) {
    let status_text = match status {
        200 => "OK",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n\
         {}",
        status, status_text, content_type, body.len(), body
    );

    let _ = socket.write_all(response.as_bytes()).await;
    let _ = socket.flush().await;
}
