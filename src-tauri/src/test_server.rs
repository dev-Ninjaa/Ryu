use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{json, Value};
use base64::Engine;

#[derive(Clone)]
struct User {
    id: usize,
    name: String,
    email: String,
}

struct ServerState {
    users: Vec<User>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            users: vec![
                User { id: 1, name: "Ninja".to_string(), email: "ninja@example.com".to_string() },
                User { id: 2, name: "Rex".to_string(), email: "rex@example.com".to_string() },
                User { id: 3, name: "Ace".to_string(), email: "ace@example.com".to_string() },
            ],
        }
    }
}

pub fn start_test_server() {
    tauri::async_runtime::spawn(async move {
        let addr = "127.0.0.1:3000";
        let listener = match TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[Test API] Could not bind to {}: {}. External test API or port in use.", addr, e);
                return;
            }
        };
        println!("[Test API] Running at http://{}", addr);

        let state = Arc::new(Mutex::new(ServerState::new()));

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let state_clone = Arc::clone(&state);
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = handle_connection(stream, state_clone).await {
                            // Connection closed or error
                            let _ = e;
                        }
                    });
                }
                Err(_) => break,
            }
        }
    });
}

async fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<ServerState>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = Vec::new();
    let mut temp = [0u8; 4096];

    let mut body_start = None;
    let mut content_length = 0;

    loop {
        let n = stream.read(&mut temp).await?;
        if n == 0 {
            break;
        }
        buffer.extend_from_slice(&temp[..n]);

        if body_start.is_none() {
            if let Some(pos) = find_subslice(&buffer, b"\r\n\r\n") {
                body_start = Some(pos + 4);
                let headers_str = String::from_utf8_lossy(&buffer[..pos]);
                for line in headers_str.lines() {
                    let lower = line.to_lowercase();
                    if lower.starts_with("content-length:") {
                        if let Some(val) = line.split(':').nth(1) {
                            content_length = val.trim().parse::<usize>().unwrap_or(0);
                        }
                    }
                }
            }
        }

        if let Some(start) = body_start {
            if buffer.len() >= start + content_length {
                break;
            }
        }

        // Safety limit to avoid huge requests
        if buffer.len() > 1_000_000 {
            break;
        }
    }

    if buffer.is_empty() {
        return Ok(());
    }

    let body_offset = body_start.unwrap_or(buffer.len());
    let raw_headers = String::from_utf8_lossy(&buffer[..body_offset.min(buffer.len())]);
    let raw_body = if body_offset < buffer.len() {
        String::from_utf8_lossy(&buffer[body_offset..]).to_string()
    } else {
        String::new()
    };

    let mut lines = raw_headers.lines();
    let request_line = match lines.next() {
        Some(l) => l,
        None => return Ok(()),
    };

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Ok(());
    }

    let method = parts[0].to_uppercase();
    let full_path = parts[1];

    let mut headers = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_lowercase(), v.trim().to_string());
        }
    }

    let (path, query_map) = parse_url(full_path);

    let (status, status_text, body_json) = route_request(&method, &path, &query_map, &headers, &raw_body, state);

    let response_body = body_json.to_string();
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: *\r\nAccess-Control-Allow-Methods: *\r\nConnection: close\r\n\r\n{}",
        status, status_text, response_body.len(), response_body
    );

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

fn route_request(
    method: &str,
    path: &str,
    query: &HashMap<String, String>,
    headers: &HashMap<String, String>,
    raw_body: &str,
    state: Arc<Mutex<ServerState>>,
) -> (u16, &'static str, Value) {
    if method == "OPTIONS" {
        return (200, "OK", json!({}));
    }

    match (method, path) {
        // GET /api/users
        ("GET", "/api/users") => {
            let st = state.lock().unwrap();
            let users_json: Vec<Value> = st.users.iter().map(|u| {
                json!({
                    "id": u.id,
                    "name": u.name,
                    "email": u.email
                })
            }).collect();
            (200, "OK", json!(users_json))
        }

        // POST /api/users
        ("POST", "/api/users") => {
            let parsed_body: Result<Value, _> = serde_json::from_str(raw_body);
            let name = match parsed_body {
                Ok(v) => v.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()),
                Err(_) => None,
            };

            if let Some(name_str) = name {
                let mut st = state.lock().unwrap();
                let new_id = st.users.len() + 1;
                let email_slug = name_str.to_lowercase().replace(' ', ".");
                let email = format!("{}@example.com", email_slug);
                let new_user = User {
                    id: new_id,
                    name: name_str.clone(),
                    email: email.clone(),
                };
                st.users.push(new_user);
                (201, "Created", json!({
                    "id": new_id,
                    "name": name_str,
                    "email": email
                }))
            } else {
                (400, "Bad Request", json!({
                    "error": "Bad Request",
                    "message": "Missing 'name' in request body"
                }))
            }
        }

        // GET /api/error
        ("GET", "/api/error") => {
            (500, "Internal Server Error", json!({
                "error": "Internal Server Error",
                "message": "This is a deterministic error for testing."
            }))
        }

        // GET /api/params
        ("GET", "/api/params") => {
            (200, "OK", json!({
                "query": query
            }))
        }

        // GET /api/headers
        ("GET", "/api/headers") => {
            (200, "OK", json!({
                "headers": headers
            }))
        }

        // POST /api/body
        ("POST", "/api/body") => {
            let content_type = headers.get("content-type").cloned().unwrap_or_default();
            let is_json = content_type.contains("application/json");

            let parsed: Result<Value, _> = serde_json::from_str(raw_body);

            if is_json && parsed.is_err() {
                return (400, "Bad Request", json!({
                    "error": "Invalid JSON",
                    "message": "The provided body is not valid JSON"
                }));
            }

            let body_val = parsed.unwrap_or_else(|_| json!(raw_body));
            (200, "OK", json!({
                "body": body_val,
                "contentType": content_type
            }))
        }

        // GET /api/auth/bearer
        ("GET", "/api/auth/bearer") => {
            let auth = headers.get("authorization");
            if let Some(auth_val) = auth {
                if auth_val.starts_with("Bearer ") {
                    let token = auth_val["Bearer ".len()..].trim();
                    return (200, "OK", json!({
                        "message": "Authenticated with Bearer token",
                        "token": token
                    }));
                }
            }
            (401, "Unauthorized", json!({
                "error": "Unauthorized",
                "message": "Missing or invalid Bearer token"
            }))
        }

        // GET /api/auth/basic
        ("GET", "/api/auth/basic") => {
            let auth = headers.get("authorization");
            if let Some(auth_val) = auth {
                if auth_val.starts_with("Basic ") {
                    let encoded = auth_val["Basic ".len()..].trim();
                    if let Ok(decoded_bytes) = base64::engine::general_purpose::STANDARD.decode(encoded) {
                        if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                            let mut parts = decoded_str.splitn(2, ':');
                            let username = parts.next().unwrap_or("");
                            return (200, "OK", json!({
                                "message": "Authenticated with Basic auth",
                                "username": username
                            }));
                        }
                    }
                }
            }
            (401, "Unauthorized", json!({
                "error": "Unauthorized",
                "message": "Missing or invalid Basic auth"
            }))
        }

        // GET /api/auth/apikey
        ("GET", "/api/auth/apikey") => {
            let key_header = headers.get("x-api-key");
            let key_query = query.get("api_key");

            if key_header.is_none() && key_query.is_none() {
                return (403, "Forbidden", json!({
                    "error": "Forbidden",
                    "message": "Missing API key in header (x-api-key) or query (api_key)"
                }));
            }

            let (key, loc) = if let Some(k) = key_header {
                (k.clone(), "header")
            } else {
                (key_query.cloned().unwrap_or_default(), "query")
            };

            (200, "OK", json!({
                "message": "Authenticated with API key",
                "key": key,
                "location": loc
            }))
        }

        // POST /api/mixed
        ("POST", "/api/mixed") => {
            let has_auth = headers.contains_key("authorization")
                || headers.contains_key("x-api-key")
                || query.contains_key("api_key");

            let parsed_body: Value = serde_json::from_str(raw_body).unwrap_or_else(|_| json!(raw_body));

            (200, "OK", json!({
                "method": method,
                "headers": headers,
                "query": query,
                "body": parsed_body,
                "auth": if has_auth { "Present" } else { "Missing" }
            }))
        }

        // Catch-all for /api/echo
        (_, p) if p == "/api/echo" => {
            let parsed_body: Value = serde_json::from_str(raw_body).unwrap_or_else(|_| json!(raw_body));
            (200, "OK", json!({
                "method": method,
                "headers": headers,
                "query": query,
                "body": parsed_body,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }

        // Handle /api/env/:value
        ("GET", p) if p.starts_with("/api/env/") => {
            let val = &p["/api/env/".len()..];
            let decoded_val = urlencoding::decode(val).unwrap_or_else(|_| std::borrow::Cow::Borrowed(val));
            (200, "OK", json!({
                "value": decoded_val
            }))
        }

        _ => {
            (404, "Not Found", json!({
                "error": "Not Found",
                "message": format!("Route {} {} not found", method, path)
            }))
        }
    }
}

fn parse_url(full_path: &str) -> (String, HashMap<String, String>) {
    let mut query = HashMap::new();
    let parts: Vec<&str> = full_path.splitn(2, '?').collect();
    let path = parts[0].to_string();

    if parts.len() > 1 {
        for pair in parts[1].split('&') {
            if pair.is_empty() {
                continue;
            }
            let mut kv = pair.splitn(2, '=');
            let key = kv.next().unwrap_or("");
            let val = kv.next().unwrap_or("");
            let dec_key = urlencoding::decode(key).unwrap_or_else(|_| std::borrow::Cow::Borrowed(key)).to_string();
            let dec_val = urlencoding::decode(val).unwrap_or_else(|_| std::borrow::Cow::Borrowed(val)).to_string();
            query.insert(dec_key, dec_val);
        }
    }

    (path, query)
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_users_route() {
        let state = Arc::new(Mutex::new(ServerState::new()));
        let (status, _, body) = route_request("GET", "/api/users", &HashMap::new(), &HashMap::new(), "", state);
        assert_eq!(status, 200);
        assert!(body.as_array().unwrap().len() >= 3);
    }

    #[test]
    fn test_error_route() {
        let state = Arc::new(Mutex::new(ServerState::new()));
        let (status, _, body) = route_request("GET", "/api/error", &HashMap::new(), &HashMap::new(), "", state);
        assert_eq!(status, 500);
        assert_eq!(body["error"], "Internal Server Error");
    }

    #[test]
    fn test_bearer_auth_route() {
        let state = Arc::new(Mutex::new(ServerState::new()));
        let mut headers = HashMap::new();
        headers.insert("authorization".to_string(), "Bearer secret-token-123".to_string());
        let (status, _, body) = route_request("GET", "/api/auth/bearer", &HashMap::new(), &headers, "", state);
        assert_eq!(status, 200);
        assert_eq!(body["token"], "secret-token-123");
    }
}
