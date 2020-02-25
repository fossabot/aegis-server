extern crate tokio;
extern crate futures;
extern crate config;

use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Get configuration settings
    let mut settings = config::Config::default();
    settings
        // Add in settings from the environment (with a prefix of AEGIS_SERVER)
        // Eg.. `AEGIS_SERVER_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("AEGIS_SERVER")).unwrap();

    // Generate hashmap of settings
    let settingsmap = settings.try_into::<HashMap<String, String>>().unwrap();

    println!("{:?}", settingsmap);

    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method.
    let server = {
        async move {
            let mut incoming = listener.incoming();
            while let Some(conn) = incoming.next().await {
                match conn {
                    Err(e) => eprintln!("accept failed = {:?}", e),
                    Ok(mut sock) => {
                    // Spawn the future that echos the data and returns how
                    // many bytes were copied as a concurrent task.
                    tokio::spawn(async move {
                            // Split up the reading and writing parts of the
                            // socket.
                            let (mut reader, mut writer) = sock.split();

                            match tokio::io::copy(&mut reader, &mut writer).await {
                                Ok(amt) => {
                                    println!("wrote {} bytes", amt);
                                }
                                Err(err) => {
                                    eprintln!("IO error {:?}", err);
                                }
                            }
                        });
                    }
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    // Start the server and block this async fn until `server` spins down.
    server.await;
}
