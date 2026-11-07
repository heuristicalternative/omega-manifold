use omega_manifold::conservation_checker::ConservationChecker;

#[test]
fn test_conservation_checker_35d() {
    let checker = ConservationChecker::new(35, 1e-5);
    let valid_tensor = vec![1.0f32; 35];
    
    let result = checker.verify_tensor_conservation(&valid_tensor);
    assert!(result.is_ok());
    assert!(result.unwrap());
}
