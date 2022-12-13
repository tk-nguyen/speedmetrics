use color_eyre::Result;
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

const METRICS_ENDPOINT: &'static str = "/metrics";
const DEFAULT_ENDPOINT: &'static str = "/";
const LISTEN_ADDRESS: &'static str = "0.0.0.0:9027";

pub fn spawn_server(registry: Registry) -> Result<()> {
    let listener = TcpListener::bind(LISTEN_ADDRESS)?;
    // So the registry can be cloned
    let thread_registry = Arc::new(Mutex::new(registry));
    for client in listener.incoming() {
        let client = client?;
        let mut reader = BufReader::new(client.try_clone()?);
        let mut writer = BufWriter::new(client.try_clone()?);
        let registry = thread_registry.clone();
        thread::spawn(move || {
            let mut request = String::new();
            reader
                .read_line(&mut request)
                .expect("Cannot read request from client!");
            match request {
                request if request.contains(&format!("GET {METRICS_ENDPOINT} HTTP/1.1\r\n")) => {
                    let mut res = vec![];
                    encode(&mut res, &registry.lock().unwrap()).unwrap();
                    write!(
                        &mut writer,
                        "{}{}{}{}",
                        concat!("HTTP/1.1 200 OK\r\n", "Content-Type: text/plain\r\n"),
                        format!("Content-Length: {}\r\n", res.len()),
                        "\r\n",
                        format!("{}\r\n", String::from_utf8_lossy(&res))
                    )
                    .expect("Cannot respond to client!")
                }
                request if request.contains(&format!("GET {DEFAULT_ENDPOINT} HTTP/1.1\r\n")) => {
                    let res = "<!doctype HTML>\n\
                        <html lang=\"en\">\n\
                        <head>\n\
                            <title>Speedtest metrics</title>\n\
                        </head>\n\
                        <body>\n\
                            <a href=\"/metrics\">Speedtest metrics</a>\n\
                        </body>\n\
                        </html>";
                    write!(
                        &mut writer,
                        "{}{}{}{}",
                        concat!("HTTP/1.1 200 OK\r\n", "Content-Type: text/html\r\n"),
                        format!("Content-Length: {}\r\n", res.len()),
                        "\r\n",
                        format!("{}\r\n", res)
                    )
                    .expect("Cannot respond to client!")
                }
                _ => {
                    let res = "<!doctype HTML>\n\
                        <html lang=\"en\">\n\
                        <head>\n\
                            <title>Speedtest metrics</title>\n\
                        </head>\n\
                        <body>\n\
                            <p>Invalid request.</p>\n\
                        </body>\n\
                        </html>";
                    write!(
                        &mut writer,
                        "{}{}{}{}",
                        concat!(
                            "HTTP/1.1 400 Bad Request\r\n",
                            "Content-Type: text/html\r\n"
                        ),
                        format!("Content-Length: {}\r\n", res.len()),
                        "\r\n",
                        format!("{}\r\n", res)
                    )
                    .expect("Cannot respond to client!")
                }
            }
        });
    }
    Ok(())
}
