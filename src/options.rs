use libp2p::{Multiaddr, PeerId};
use structopt::StructOpt;

use crate::networks::DHTNetwork;

#[derive(Debug, StructOpt)]
pub(crate) enum Options {
    /// Lookup peer by it's libp2p address
    Direct {
        /// Peer address (Multi Address Format)
        #[structopt(long, short)]
        address: Multiaddr,
    },
    /// Lookup peer by its public ID via Kademlia DHT
    Dht {
        /// Peer ID
        #[structopt(long, short)]
        peer_id: PeerId,
        /// DHT network of the peer
        #[structopt(long, short)]
        dht_network: DHTNetwork,
    },
}
