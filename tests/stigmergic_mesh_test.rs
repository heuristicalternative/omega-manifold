use omega_manifold::stigmergic_mesh::StigmergicMesh;

#[test]
fn test_stigmergic_pheromone_propagation() {
    let mut mesh = StigmergicMesh::new("Melusine-Mesh-Node");
    
    mesh.deposit_trace("free_energy_gradient", 0.75);
    
    assert_eq!(mesh.read_trace("free_energy_gradient"), Some(&0.75));
    assert_eq!(mesh.read_trace("non_existent_trace"), None);
}
