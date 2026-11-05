use omega_manifold::gossip::GossipProtocol;

#[test]
fn test_gossip_peer_broadcasting() {
    let mut gossip = GossipProtocol::new("Melusine-Node-01");
    gossip.register_peer("Peer-Node-Alpha");
    gossip.register_peer("Peer-Node-Beta");

    assert_eq!(gossip.active_peer_count(), 2);

    let messages = gossip.broadcast_message("free_energy_state_sync");
    assert_eq!(messages.len(), 2);
}
