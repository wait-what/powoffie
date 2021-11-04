use serde::Deserialize;
use std::{collections::HashMap, time::Instant};
use tiny_http::{Header, Method, Response, Server};

#[derive(Debug, Deserialize)]
struct Config {
    bind: String,
    relay_endpoint: String,
    host_endpoint: String,
    rate_limit: u64,
    tokens: HashMap<String, String>,
}

fn main() {
    let json_file = std::fs::read_to_string("./config.json").expect("Couldn't read config.json");
    let config: Config = serde_json::from_str(&json_file).expect("Couldn't parse config.json");

    let server = Server::http(config.bind.as_str()).unwrap();
    let mut rate_limits: HashMap<String, Instant> = HashMap::new();
    let mut last_winner = "undefined".to_string();

    println!("Running powoffie-relay on {}", config.bind.as_str());

    for mut request in server.incoming_requests() {
        match request.method() {
            Method::Get => {
                if request.url() == "/" {
                    let html = include_str!("./index.html").replace(
                        "{{endpoint}}",
                        format!("{}/poweroff", config.relay_endpoint.as_str()).as_str(),
                    );
                    let html_header =
                        Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();

                    request
                        .respond(Response::from_string(html).with_header(html_header.clone()))
                        .unwrap();

                    continue;
                } else if request.url() == "/winner" {
                };
            }
            Method::Post => {
                if request.url() == "/poweroff" {
                    let mut password = String::new();
                    request.as_reader().read_to_string(&mut password).unwrap();

                    let token = match request
                        .headers()
                        .iter()
                        .find(|h: &&Header| h.field.equiv("Authorization"))
                    {
                        Some(header) => header.value.as_str(),
                        None => {
                            request.respond(Response::empty(401)).unwrap();
                            continue;
                        }
                    };

                    if !config.tokens.contains_key(token) {
                        println!("=> Invalid token: {}", token);
                        request.respond(Response::empty(401)).unwrap();
                        continue;
                    };

                    println!("=> Accepted token: {}", token);

                    let now = Instant::now();
                    if rate_limits.contains_key(token) {
                        let limit = rate_limits[token];
                        if now < limit {
                            println!(" > Rate limited");
                            request
                                .respond(
                                    Response::from_string((limit - now).as_secs().to_string())
                                        .with_status_code(429),
                                )
                                .unwrap();
                            continue;
                        };
                    };
                    rate_limits.insert(
                        token.to_string(),
                        now + std::time::Duration::from_secs(config.rate_limit),
                    );

                    // Send password to host
                    println!(" > Relaying to host");

                    let host_response = chttp::post(
                        format!("{}/poweroff", config.host_endpoint.as_str()).as_str(),
                        password,
                    );

                    match host_response {
                        Ok(host_response) => {
                            use chttp::http::StatusCode;

                            match host_response.status() {
                                StatusCode::OK => {
                                    println!(" > Success");
                                    last_winner =
                                        config.tokens.get(token.clone()).unwrap().to_string();
                                    request.respond(Response::empty(200)).unwrap();
                                    continue;
                                }
                                StatusCode::UNAUTHORIZED => {
                                    println!(" > Host rejected password");
                                    request.respond(Response::empty(403)).unwrap();
                                    continue;
                                }
                                StatusCode::GONE => {
                                    println!(" > Host already activated");
                                    request
                                        .respond(
                                            Response::from_string(last_winner.clone())
                                                .with_status_code(410),
                                        )
                                        .unwrap();
                                    continue;
                                }
                                _ => {
                                    println!(" > Host error");
                                    request.respond(Response::empty(500)).unwrap();
                                    continue;
                                }
                            }
                        }
                        Err(_) => {
                            println!(" > Couldn't connect to host");
                            request
                                .respond(
                                    Response::from_string(last_winner.clone())
                                        .with_status_code(410),
                                )
                                .unwrap();
                            continue;
                        }
                    };
                }
            }
            _ => (),
        }

        request.respond(Response::empty(404)).unwrap();
    }
}
