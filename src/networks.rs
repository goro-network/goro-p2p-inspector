use libp2p::{Multiaddr, PeerId};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub(crate) enum DHTNetwork {
    GoRo,
    Ipfs,
    Krigan,
}

impl FromStr for DHTNetwork {
    type Err = String;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source.to_lowercase().as_str() {
            "goro" => Ok(Self::GoRo),
            "ipfs" => Ok(Self::Ipfs),
            "krigan" => Ok(Self::Krigan),
            _ => Err("Supported networks are GoRo, IPFS, and Krigan.".to_owned()),
        }
    }
}

impl DHTNetwork {
    pub(crate) fn bootnodes(&self) -> Vec<(Multiaddr, PeerId)> {
        match self {
            Self::GoRo => {
                vec![(
                    "/ip4/108.136.33.203/tcp/21212"
                        .parse()
                        .expect("Bad MultiAddr!"),
                    "12D3KooWPHzn5X8uGTSZKTHBgfAyMViJZGwpKaej26gXNsLuGKua"
                        .parse()
                        .expect("Bad PeerID!"),
                )]
            }
            Self::Ipfs => {
                vec![
                    (
                        "/ip4/104.131.131.82/tcp/4001"
                            .parse()
                            .expect("Bad MultiAddr!"),
                        "QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ"
                            .parse()
                            .expect("Bad PeerID!"),
                    ),
                    (
                        "/dnsaddr/bootstrap.libp2p.io"
                            .parse()
                            .expect("Bad MultiAddr!"),
                        "QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN"
                            .parse()
                            .expect("Bad PeerID!"),
                    ),
                    (
                        "/dnsaddr/bootstrap.libp2p.io"
                            .parse()
                            .expect("Bad MultiAddr!"),
                        "QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa"
                            .parse()
                            .expect("Bad PeerID!"),
                    ),
                    (
                        "/dnsaddr/bootstrap.libp2p.io"
                            .parse()
                            .expect("Bad MultiAddr!"),
                        "QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
                            .parse()
                            .expect("Bad PeerID!"),
                    ),
                    (
                        "/dnsaddr/bootstrap.libp2p.io"
                            .parse()
                            .expect("Bad MultiAddr!"),
                        "QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt"
                            .parse()
                            .expect("Bad PeerID!"),
                    ),
                ]
            }
            Self::Krigan => {
                vec![(
                    "/ip4/108.136.33.203/tcp/30333"
                        .parse()
                        .expect("Bad MultiAddr!"),
                    "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2"
                        .parse()
                        .expect("Bad PeerID!"),
                )]
            }
        }
    }

    pub(crate) fn protocol(&self) -> Option<String> {
        match self {
            Self::GoRo => Some("/substrate/1.0".into()),
            Self::Ipfs => None,
            Self::Krigan => Some("/substrate/1.0".into()),
        }
    }
}
