extern crate futures;
extern crate tokio;
#[macro_use] extern crate log;

use futures::stream::StreamExt;
use std::net::SocketAddr;
use std::str;
use tokio::net::TcpListener;
use tokio_util::codec::{BytesCodec, Decoder};

mod logging;

#[tokio::main]
async fn main() {
    const SERVERADDRESS: &str = "127.0.0.1:6124";
    const SERVERLOG: &str = "/var/log/aegisserver.log";

    let server: SocketAddr = SERVERADDRESS
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
                        //eprintln!("accept failed = {:?}", e);
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
                                    Ok(bytes) => info!("{:#?}", bytes),
                                    Err(err) => println!("Socket closed with error: {:#?}", err),
                                }
                            }
                        });
                    }
                }
            }
        }
    };

    println!("{}", format!("Server listening on {}", SERVERADDRESS));

    // Start the server and block this async fn until `server` spins down.
    server.await;
}
