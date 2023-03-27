use crate::logging::{log_debug, log_info};
use crate::options::Options;
use futures::executor::block_on;
use futures::future::Either;
use futures::StreamExt;
use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::OrTransport as TCPOrTransport;
use libp2p::core::upgrade::{SelectUpgrade, Version as UpgradeVersion};
use libp2p::core::ConnectedPoint;
use libp2p::dns::DnsConfig;
use libp2p::identify::{
    Behaviour as IdentifyBehaviour, Config as IdentifyConfig, Event as IdentityEvent,
    Info as IdentityInfo,
};
use libp2p::identity::{Keypair, PublicKey};
use libp2p::kad::store::MemoryStore;
use libp2p::kad::{
    GetClosestPeersOk as KademliatGetClosestPeersOk, Kademlia, KademliaConfig, KademliaEvent,
    ProgressStep as KademliaProgressStep, QueryResult as KademliaQueryResult,
};
use libp2p::mplex::{MaxBufferBehaviour as MultiplexMaxBufferBehaviour, MplexConfig};
use libp2p::noise::{Keypair as NoiseKeypair, NoiseConfig, X25519Spec};
use libp2p::ping::{Behaviour as PingBehaviour, Config as PingConfig};
use libp2p::quic::tokio::Transport as QUICTransport;
use libp2p::quic::Config as QUICConfig;
use libp2p::relay::client::{new as new_relay_client, Behaviour as RelayClientBehaviour};
use libp2p::swarm::keep_alive::Behaviour as SwarmKeepAliveBehaviour;
use libp2p::swarm::{
    DialError as SwarmDialError, NetworkBehaviour as SwarmNetworkBehaviour, SwarmBuilder,
    SwarmEvent,
};
use libp2p::tcp::async_io::Transport as AsyncTCPTransport;
use libp2p::tcp::Config as TCPConfig;
use libp2p::yamux::{WindowUpdateMode as YamuxWindowUpdateMode, YamuxConfig};
use libp2p::{InboundUpgradeExt, Multiaddr, OutboundUpgradeExt, PeerId, Swarm, Transport};
use std::fmt::{Display, Formatter, Result as FormatterResult};
use structopt::StructOpt;
use thiserror::Error;
use tokio::time::Duration;

pub(crate) type PeerLookupResult = Result<PeerInfo, PeerLookupError>;

#[derive(Debug, Error)]
pub(crate) enum PeerLookupError {
    #[error(transparent)]
    FailedToDialPeer(#[from] SwarmDialError),
    #[error("Failed to find peer on DHT")]
    FailedToFindPeerOnDht,
}

#[derive(SwarmNetworkBehaviour)]
pub(crate) struct PeerLookupBehaviour {
    pub(crate) dht: Kademlia<MemoryStore>,
    pub(crate) ping: PingBehaviour,
    pub(crate) identify: IdentifyBehaviour,
    pub(crate) relay: RelayClientBehaviour,
    pub(crate) keep_alive: SwarmKeepAliveBehaviour,
}

pub(crate) struct PeerInfo {
    id: PeerId,
    public_key: PublicKey,
    protocol_version: String,
    agent_version: String,
    listen_addresses: Vec<Multiaddr>,
    protocols: Vec<String>,
    observed_address: Multiaddr,
}

impl From<IdentityInfo> for PeerInfo {
    fn from(value: IdentityInfo) -> Self {
        Self {
            id: value.public_key.to_peer_id(),
            public_key: value.public_key,
            protocol_version: value.protocol_version,
            agent_version: value.agent_version,
            listen_addresses: value.listen_addrs,
            protocols: value.protocols,
            observed_address: value.observed_addr,
        }
    }
}

impl Display for PeerInfo {
    fn fmt(&self, formatter: &mut Formatter) -> FormatterResult {
        writeln!(formatter, "[Peer ID]\n  \"{}\"", &self.id)?;
        writeln!(formatter, "[Public Key]\n  \"{:?}\"", &self.public_key)?;
        writeln!(
            formatter,
            "[Protocol Version]\n  \"{}\"",
            &self.protocol_version
        )?;
        writeln!(formatter, "[Agent Version]\n  \"{}\"", &self.agent_version)?;
        writeln!(
            formatter,
            "[Observed Address]\n  \"{}\"",
            &self.observed_address
        )?;

        if !self.listen_addresses.is_empty() {
            writeln!(formatter, "[Listen Addresses]")?;

            for addr in &self.listen_addresses {
                writeln!(formatter, "  - \"{addr}\"")?;
            }
        }

        if !self.protocols.is_empty() {
            writeln!(formatter, "[Protocols]")?;

            for protocol in &self.protocols {
                writeln!(formatter, "  - \"{protocol}\"")?;
            }
        }

        Ok(())
    }
}

pub(crate) enum PeerLookupClient {
    Direct {
        swarm: Swarm<PeerLookupBehaviour>,
        destination: Multiaddr,
    },
    Dht {
        swarm: Swarm<PeerLookupBehaviour>,
        peer_id: PeerId,
    },
}

impl PeerLookupClient {
    async fn wait_for_indentication(
        swarm: &mut Swarm<PeerLookupBehaviour>,
        peer: PeerId,
    ) -> PeerLookupResult {
        loop {
            match swarm
                .next()
                .await
                .expect("Programmatic error: infinite streams!")
            {
                SwarmEvent::Behaviour(PeerLookupBehaviourEvent::Identify(
                    IdentityEvent::Received { peer_id, info },
                )) => {
                    if peer_id == peer {
                        return Ok(info.into());
                    }
                }
                other_event => log_debug!("{other_event:?}"),
            }
        }
    }

    async fn lookup_directly(
        swarm: &mut Swarm<PeerLookupBehaviour>,
        destination_address: Multiaddr,
    ) -> PeerLookupResult {
        swarm.dial(destination_address.clone())?;

        loop {
            match swarm
                .next()
                .await
                .expect("Programmatic error: infinite streams!")
            {
                SwarmEvent::ConnectionEstablished {
                    peer_id,
                    endpoint,
                    num_established,
                    established_in,
                    ..
                } => {
                    assert_eq!(Into::<u32>::into(num_established), 1);

                    match endpoint {
                        ConnectedPoint::Dialer {
                            address,
                            role_override,
                        } => {
                            if address == destination_address {
                                log_info!("Connection established in {established_in:?} for \"{address}\" (role override: {role_override:?})");

                                return Self::wait_for_indentication(swarm, peer_id).await;
                            }
                        }
                        ConnectedPoint::Listener {
                            local_addr,
                            send_back_addr,
                        } => {
                            log_debug!("Connected to listener of {local_addr} where send back address is {send_back_addr}, skipping...");
                        }
                    }
                }
                SwarmEvent::OutgoingConnectionError { peer_id: _, error } => {
                    return Err(error.into());
                }
                SwarmEvent::Dialing(peer_id) => {
                    log_info!("Dialing {peer_id}");
                }
                SwarmEvent::Behaviour(_) => {
                    // Ignore any behaviour events until we are connected to the
                    // destination peer. These should be events from the
                    // connection to a relay only.
                }
                other_event => log_debug!("{other_event:?}"),
            }
        }
    }

    async fn lookup_via_dht(
        swarm: &mut Swarm<PeerLookupBehaviour>,
        target_peer_id: PeerId,
    ) -> PeerLookupResult {
        swarm.behaviour_mut().dht.get_closest_peers(target_peer_id);

        loop {
            match swarm
                .next()
                .await
                .expect("Programmatic error: infinite streams!")
            {
                SwarmEvent::ConnectionEstablished {
                    peer_id,
                    num_established,
                    established_in,
                    endpoint,
                    ..
                } => {
                    assert_eq!(Into::<u32>::into(num_established), 1);

                    if peer_id == target_peer_id {
                        match endpoint {
                            ConnectedPoint::Dialer { address, .. } => {
                                let address_string =
                                    address.to_string().replace(&peer_id.to_string(), "");
                                log_info!("Connection established in {established_in:?} for \"{peer_id}\" (via \"{address_string}\")");
                            }
                            ConnectedPoint::Listener {
                                local_addr,
                                send_back_addr,
                            } => {
                                log_info!("Connection established in {established_in:?} for \"{peer_id}\" (via \"{local_addr}\" with send back address \"{send_back_addr}\")");
                            }
                        }

                        return Self::wait_for_indentication(swarm, peer_id).await;
                    }
                }
                SwarmEvent::Behaviour(PeerLookupBehaviourEvent::Dht(
                    KademliaEvent::OutboundQueryProgressed {
                        result: KademliaQueryResult::Bootstrap(_),
                        ..
                    },
                )) => {
                    panic!("Unexpected bootstrap in Kademlia rooting!");
                }
                SwarmEvent::Behaviour(PeerLookupBehaviourEvent::Dht(
                    KademliaEvent::OutboundQueryProgressed {
                        result:
                            KademliaQueryResult::GetClosestPeers(Ok(KademliatGetClosestPeersOk {
                                peers,
                                ..
                            })),
                        step: KademliaProgressStep { count: _, last },
                        ..
                    },
                )) => {
                    if peers.contains(&target_peer_id) {
                        if !Swarm::is_connected(swarm, &target_peer_id) {
                            Swarm::dial(swarm, target_peer_id).unwrap();
                        }

                        return Self::wait_for_indentication(swarm, target_peer_id).await;
                    }

                    if last {
                        return Err(PeerLookupError::FailedToFindPeerOnDht);
                    }
                }
                _ => {}
            }
        }
    }

    pub(crate) fn create_from_options() -> Self {
        let (destination_address, dht_network) = match Options::from_args() {
            Options::Dht {
                peer_id,
                dht_network,
            } => (None, Some((dht_network, peer_id))),
            Options::Direct { address } => (Some(address), None),
        };

        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        log_info!("Local peer id: \"{local_peer_id}\"");
        let (relay_transport, relay_client) = new_relay_client(local_peer_id);
        let transport = {
            let authentication_config = {
                let noise_keypair_spec = NoiseKeypair::<X25519Spec>::new()
                    .into_authentic(&local_key)
                    .expect("Programmatic error: incompatible keypair spec!");

                NoiseConfig::xx(noise_keypair_spec).into_authenticated()
            };
            let multiplexing_config = {
                let mut mplex_config = MplexConfig::new();
                mplex_config.set_max_buffer_behaviour(MultiplexMaxBufferBehaviour::Block);
                mplex_config.set_max_buffer_size(usize::MAX);
                let mut yamux_config = YamuxConfig::default();
                yamux_config.set_window_update_mode(YamuxWindowUpdateMode::on_read());

                SelectUpgrade::new(yamux_config, mplex_config)
                    .map_inbound(StreamMuxerBox::new)
                    .map_outbound(StreamMuxerBox::new)
            };
            let tcp_and_relay_transport = TCPOrTransport::new(
                relay_transport,
                AsyncTCPTransport::new(TCPConfig::new().port_reuse(true).nodelay(true)),
            )
            .upgrade(UpgradeVersion::V1)
            .authenticate(authentication_config)
            .multiplex(multiplexing_config)
            .timeout(Duration::from_secs(10));
            let quic_transport = {
                let mut config = QUICConfig::new(&local_key);
                config.support_draft_29 = true;

                QUICTransport::new(config)
            };

            block_on(DnsConfig::system(TCPOrTransport::new(
                quic_transport,
                tcp_and_relay_transport,
            )))
            .expect("Can't decide network to connect! Does the network available?")
            .map(|either_output, _| match either_output {
                Either::Left((peer_id, stream_muxer)) => {
                    (peer_id, StreamMuxerBox::new(stream_muxer))
                }
                Either::Right((peer_id, stream_muxer)) => {
                    (peer_id, StreamMuxerBox::new(stream_muxer))
                }
            })
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
            .boxed()
        };

        let behaviour = {
            let local_peer_id = PeerId::from(local_key.public());
            let store = MemoryStore::new(local_peer_id);
            let mut kademlia_config = KademliaConfig::default();

            if let Some(protocol_name) = dht_network
                .as_ref()
                .and_then(|(dht_network, _)| dht_network.protocol())
            {
                kademlia_config.set_protocol_names(vec![protocol_name.into_bytes().into()]);
            }

            let kademlia = Kademlia::with_config(local_peer_id, store, kademlia_config);
            let ping = PingBehaviour::new(PingConfig::new());
            let user_agent = "substrate-node/v2.0.0-85dacde-wasm32 (unknown)".to_owned();
            let proto_version = "/substrate/1.0".to_string();
            let identify = IdentifyBehaviour::new(
                IdentifyConfig::new(proto_version, local_key.public())
                    .with_agent_version(user_agent),
            );

            PeerLookupBehaviour {
                dht: kademlia,
                ping,
                identify,
                relay: relay_client,
                keep_alive: SwarmKeepAliveBehaviour,
            }
        };

        let mut swarm =
            SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id).build();

        if let Some((network, peer_id)) = dht_network {
            for (addr, peer_id) in network.bootnodes() {
                swarm.behaviour_mut().dht.add_address(&peer_id, addr);
            }

            Self::Dht { swarm, peer_id }
        } else {
            Self::Direct {
                swarm,
                destination: destination_address.expect("Programmatic error: bad logical flow!"),
            }
        }
    }

    pub(crate) async fn do_lookup(self) -> PeerLookupResult {
        match self {
            Self::Direct {
                mut swarm,
                destination,
            } => Self::lookup_directly(&mut swarm, destination).await,
            Self::Dht { mut swarm, peer_id } => Self::lookup_via_dht(&mut swarm, peer_id).await,
        }
    }
}
