use std::{error::Error, time::Duration};

use anyhow::Result;
use futures::prelude::*;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{
    Multiaddr, identify, kad,
    kad::{Mode, store::MemoryStore},
    mdns, noise, ping,
    swarm::SwarmEvent,
    tcp, yamux,
};
use tracing_subscriber::EnvFilter;

#[derive(NetworkBehaviour)]
struct GameNet {
    mdns: mdns::tokio::Behaviour,  // Discover local peers
    identify: identify::Behaviour, // Allows peer identity
    // gossipsub: gossipsub::Behaviour, // Publish info across the network
    kad: kad::Behaviour<MemoryStore>, // Store info on the network
}

pub async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?;
    let mut swarm = swarm
        .with_behaviour(|key| {
            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            let identify =
                identify::Behaviour::new(identify::Config::new("0.0.1".to_string(), key.public()));
            let kad = kad::Behaviour::new(
                key.public().to_peer_id(),
                MemoryStore::new(key.public().to_peer_id()),
            );
            Ok(GameNet {
                mdns,
                identify,
                kad,
            })
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))) // Allows us to observe pings indefinitely.
        .build();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {}
        }
    }
}
