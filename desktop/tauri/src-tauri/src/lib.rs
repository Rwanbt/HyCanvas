//! Isolated desktop proof of concept. It deliberately does not start a remote
//! service, change HyCanvas's backend, or grant any frontend native privileges.

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};
use tauri::{webview::WebviewWindowBuilder, WebviewUrl};

const LOCAL_ENDPOINT: &str = "http://127.0.0.1:8005/";

/// Check an actual HyCanvas ready endpoint, rather than just an occupied port.
fn local_hycanvas_ready() -> bool {
    let addr: SocketAddr = "127.0.0.1:8005".parse().expect("valid loopback address");
    let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(400)) else {
        return false;
    };
    if stream
        .set_read_timeout(Some(Duration::from_millis(600)))
        .is_err()
        || stream
            .set_write_timeout(Some(Duration::from_millis(600)))
            .is_err()
    {
        return false;
    }
    if stream
        .write_all(b"GET /readyz HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .is_err()
    {
        return false;
    }
    let mut response = String::new();
    if stream.read_to_string(&mut response).is_err() {
        return false;
    }
    response.starts_with("HTTP/1.1 200") && response.contains("\"status\":\"ready\"")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // The existing Go build embeds the Next.js export and serves it on
            // the same origin as the API. Do not load a hosted website.
            let url = if local_hycanvas_ready() {
                WebviewUrl::External(LOCAL_ENDPOINT.parse()?)
            } else {
                WebviewUrl::App("index.html".into())
            };
            WebviewWindowBuilder::new(app, "main", url)
                .title("HyCanvas Desktop Preview")
                .inner_size(1250.0, 800.0)
                .min_inner_size(800.0, 560.0)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("unable to start HyCanvas Desktop Preview");
}
