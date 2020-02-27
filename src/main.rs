extern crate futures;
extern crate tokio;
#[macro_use] extern crate log;

use futures::stream::StreamExt;
use std::net::SocketAddr;
use std::str;
use tokio::net::TcpListener;
use tokio_util::codec::{BytesCodec, Decoder};

mod logging;
mod protocol;
mod configuration;

#[tokio::main]
async fn main() {
    // Load config
    let conf = configuration::parse_config();
    let server_ip: &str = &conf.ip;
    
    const SERVERLOG: &str = "/var/log/aegisserver.log";
    
    let server: SocketAddr = server_ip
        .parse()
        .expect("Unable to parse socket address");

    logging::logger(SERVERLOG);
    
    let mut listener = TcpListener::bind(server).await.unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method.
    let server = {
        async move {
            let mut incoming = listener.incoming();
            while let Some(conn) = incoming.next().await {
                match conn {
                    Err(e) => {
                        debug!("accept failed = {:?}", e);
                    },
                    Ok(sock) => {
                        // Spawn the future that echos the data and returns how
                        // many bytes were copied as a concurrent task.
                        tokio::spawn(async move {
                            let mut framed = BytesCodec::new().framed(sock);
                            debug!("Socket input");
                            
                            while let Some(message) = framed.next().await {
                                match message {
                                    Ok(bytes) => {
                                        info!("{:#?}", bytes);
                                        let msg = protocol::message_from_data(&bytes);
                                        let msgstr = protocol::parse_message_string(msg);
                                        println!("MESSAGE: {:#?}", msg);
                                        println!("MESSAGE STRING: {:#?}", msgstr);
                                    },
                                    Err(err) => println!("Socket closed with error: {:#?}", err),
                                }
                            }
                        });
                    }
                }
            }
        }
    };

    println!("{}", format!("Server listening on {}", server_ip));

    // Start the server and block this async fn until `server` spins down.
    server.await;
}
