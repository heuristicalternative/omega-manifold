use omega_manifold::gossip_protocol::GossipProtocolNode;

#[test]
fn test_gossip_protocol_node_operations() {
    let mut node = GossipProtocolNode::new("melusine-node-alpha");
    node.register_peer("100.64.0.2:9000");
    node.register_peer("100.64.0.3:9000");

    assert_eq!(node.peer_count(), 2);
    assert_eq!(node.increment_epoch(), 1);
}
