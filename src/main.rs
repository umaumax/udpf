use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use log::{debug, info};
use structopt::StructOpt;
use tokio::{net::UdpSocket, sync::mpsc};

#[derive(StructOpt)]
/// udp port forwarding command
struct Opt {
    #[structopt(default_value("0.0.0.0:10010"), help = "listening bind address")]
    listen: String,

    #[structopt(default_value("localhost:10020"), help = "forwarding bind address")]
    forward: String,

    #[structopt(long = "verbose", help = "verbose flag")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let listen_addr = opt.listen;
    let forward_addr = opt.forward;
    let verbose = opt.verbose;

    let socket = UdpSocket::bind(&listen_addr).await?;
    let socket = Arc::new(socket);

    env::set_var("RUST_LOG", "info");
    if verbose {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    info!("Listening on: {}", listen_addr);
    info!("Forwarding to: {}", forward_addr);

    let mut buf = vec![0; 1024];

    let mut sockets_map = HashMap::new();
    let (server_tx, mut server_rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(32);
    let send_back_socket = socket.clone();
    tokio::spawn(async move {
        while let Some((bytes, addr)) = server_rx.recv().await {
            let len = send_back_socket.send_to(&bytes, &addr).await?;
            debug!("{:?} bytes sended back to {:?}", len, &addr);
        }
        Ok::<(), anyhow::Error>(())
    });
    loop {
        let (len, incoming_addr) = socket.recv_from(&mut buf).await?;
        debug!("{:?} bytes received from {:?}", len, incoming_addr);

        let tx = if sockets_map.contains_key(&incoming_addr) {
            sockets_map.get(&incoming_addr).unwrap()
        } else {
            let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32);
            let forward_addr = forward_addr.clone();
            let listen_addr = listen_addr.clone();
            let server_tx = server_tx.clone();
            tokio::spawn(async move {
                let sock = UdpSocket::bind("0.0.0.0:0").await?;
                let sock = Arc::new(sock);
                let recv_sock = sock.clone();
                sock.connect(&forward_addr).await?;
                info!("bind {} access to {}", incoming_addr, sock.local_addr()?);
                info!(
                    "{} <--> [{} {}] <--> {}",
                    incoming_addr,
                    listen_addr,
                    sock.local_addr()?,
                    forward_addr
                );

                let forward_response_addr = forward_addr.clone();
                #[allow(unreachable_code)]
                tokio::spawn(async move {
                    let mut buf = vec![0; 1024];
                    loop {
                        let len = recv_sock.recv(&mut buf).await?;
                        debug!("{:?} bytes received from {:?}", len, &forward_response_addr);
                        server_tx.send((buf[..len].to_vec(), incoming_addr)).await?;
                    }
                    Ok::<(), anyhow::Error>(())
                });
                while let Some(bytes) = rx.recv().await {
                    let len = sock.send(&bytes).await?;
                    debug!("{:?} bytes sended to {:?}", len, &forward_addr);
                }
                Ok::<(), anyhow::Error>(())
            });
            sockets_map.insert(incoming_addr.clone(), tx);
            sockets_map.get(&incoming_addr).unwrap()
        };
        tx.send(buf[..len].to_vec()).await?;
    }
}
