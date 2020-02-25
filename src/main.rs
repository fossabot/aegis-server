extern crate tokio;
extern crate futures;

use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::stream::StreamExt;
use std::str;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    
    const SERVERADDRESS: &str = "127.0.0.1:6124";
    const SERVERLOG: &str =  "/var/log/aegisserver.log";
    
    let server: SocketAddr = SERVERADDRESS
        .parse()
        .expect("Unable to parse socket address");
    
    let mut listener = TcpListener::bind(server).await.unwrap();

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
                            // For debugging print whole socket
                            println!("{:#?}", sock);
                        
                            // Split up the reading and writing parts of the
                            // socket.
                            let (mut reader, mut writer) = sock.split();
                            
                            // For debugging print reader
                            println!("{:#?}", reader);

                            // For debugging print writer
                            println!("{:#?}", writer);

                        
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

    println!("{}", format!("Server listening on {}", SERVERADDRESS));

    // Start the server and block this async fn until `server` spins down.
    server.await;
}
