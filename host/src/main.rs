extern crate strip_ansi_escapes;

use std::{env::args, process::Command, thread::sleep};
use tiny_http::{Method, Response, Server};

fn main() {
    let port: u16 = args()
        .nth(1)
        .expect("Missing port")
        .parse()
        .expect("Invalid port");
    let timeout: u64 = args()
        .nth(2)
        .expect("Missing timeout")
        .parse()
        .expect("Invalid timeout");
    let password = args().nth(3).expect("Missing password");

    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
    let mut activated = false;

    println!("Running powoffie-host on :{}\nPassword: {}", port, password);

    for mut request in server.incoming_requests() {
        match request.method() {
            Method::Post => {
                if request.url() == "/poweroff" {
                    if activated {
                        request.respond(Response::empty(410)).unwrap();
                        continue;
                    };

                    let body = {
                        let mut raw_body = String::new();
                        request.as_reader().read_to_string(&mut raw_body).unwrap();

                        let clean_body = strip_ansi_escapes::strip(raw_body.as_bytes()).unwrap();

                        match String::from_utf8(clean_body) {
                            Ok(body) => body,
                            Err(_) => {
                                request.respond(Response::empty(500)).unwrap();
                                continue;
                            }
                        }
                    };

                    println!("=> {}", body);

                    if body == password {
                        activated = true;

                        request.respond(Response::empty(200)).unwrap();

                        std::thread::spawn(move || {
                            if timeout > 0 {
                                println!("! Powoffie running in {} seconds", timeout);
                                Command::new("notify-send")
                                    .arg(format!("Powoffie running in {} seconds", timeout))
                                    .spawn()
                                    .unwrap();

                                sleep(std::time::Duration::from_secs(timeout));
                            }

                            sleep(std::time::Duration::from_secs(2));

                            Command::new("sudo").arg("poweroff").spawn().unwrap();
                        });
                    } else {
                        request.respond(Response::empty(401)).unwrap();
                    }

                    continue;
                };
            }
            _ => (),
        }

        request.respond(Response::empty(404)).unwrap();
    }
}
