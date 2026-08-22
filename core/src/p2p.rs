// Copyright (c) 2026 Emirhan CAMCI. All rights reserved.

use libp2p::{
    mdns, noise, tcp, yamux,
    swarm::{NetworkBehaviour, SwarmEvent},
    identity, PeerId, SwarmBuilder
};
use std::time::Duration;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OplogEntry {
    pub id: i64,
    pub table_name: String,
    pub op_type: String, // INSERT, UPDATE, DELETE
    pub delta: String,   // JSON payload of modified columns
    pub hlc: String,     // Hybrid Logical Clock
    pub site_id: String,
}

#[derive(NetworkBehaviour)]
pub struct EdgeSyncBehaviour {
    mdns: mdns::tokio::Behaviour,
    // Note: In a complete implementation, you would add a Request/Response 
    // or Gossipsub behaviour here to actually transfer the OplogEntry payloads.
}

/// Spawns the background P2P daemon thread
pub async fn start_p2p_daemon(mut _oplog_rx: mpsc::Receiver<OplogEntry>) -> Result<(), Box<dyn std::error::Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("EdgeSync Node ID: {}", local_peer_id);

    let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new, // This provides the E2EE
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let mdns_config = mdns::Config {
                ttl: Duration::from_secs(60),
                query_interval: Duration::from_secs(10),
                enable_ipv6: false,
            };
            EdgeSyncBehaviour {
                mdns: mdns::tokio::Behaviour::new(mdns_config, key.public().to_peer_id()).unwrap(),
            }
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // Listen on all interfaces on a random OS-assigned port
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        tokio::select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(EdgeSyncBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer, addr) in list {
                        println!("Discovered local EdgeSync peer: {} @ {}", peer, addr);
                        // Handshake and trigger CRDT sync
                    }
                }
                SwarmEvent::Behaviour(EdgeSyncBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer, _) in list {
                        println!("EdgeSync peer disconnected: {}", peer);
                    }
                }
                _ => {}
            },
            
            // Read from local channel to broadcast to peers
            // entry = _oplog_rx.recv() => {
            //     // Gossip to connected peers
            // }
        }
    }
}
