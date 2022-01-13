use futures::future::Boxfuture;
use void::Void;
use futures::prelude::*;
use async_std::task;
use std::error::Error;
use libp2p::core::identity;
use std::collections::VecDeque;
use libp2p::swarm::{NetworkBehaviour, IntoProtocolsHandler, NetworkBehaviourAction, PollParameters};
use std::task::{Context, Poll};
use libp2p::{Multiaddr, PeerId};
use libp2p::core::connection::ConnectionId;

//echo protocol
struct EchoBehaviour{
    events: VecDeque<EchoBehaviourEvent>,
    config: EchoBehaviourConfig,
}
impl EchoBehaviour{
    pub fn new(config: EchoBehaviourConfig)->Self{
        return EchoBehaviour{
            events : VecDeque::new(),
            config:  config,
        };
    }
}
struct EchoBehaviourConfig{
    init_echo: bool,
}

#[derive(Debug)]
struct EchoBehaviourEvent{
    pub peer: peerId,
    pub result: EchoHandlerEvent,
}
// implement echo protocol
impl NetworkBehaviour for EchoBehaviour{
    type ProtocolsHandler = ();
    type OutEvent = ();

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        todo!()
    }

    fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<Multiaddr> {
        todo!()
    }

    fn inject_connected(&mut self, peer_id: &PeerId) {
        todo!()
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId) {
        todo!()
    }

    fn inject_event(&mut self, peer_id: PeerId, connection: ConnectionId, event: _) {
        todo!()
    }

    fn poll(&mut self, cx: &mut Context<'_>, params: &mut impl PollParameters<SupportedProtocolsIter=_, ListenedAddressesIter=_, ExternalAddressesIter=_>) -> Poll<NetworkBehaviourAction<_, Self::OutEvent>> {
        todo!()
    }
}

//
fn main()->Result<(),Box<dyn Error>>{
    //create a random peerid
    let id_keys=identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    info!("peer id :{:?}",peer_id);
    //create a transport
    let transport=libp2p::development_transport(id_keys)?;

    //define object of behaviour config
}