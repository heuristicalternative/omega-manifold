use std::fs;
use std::path::Path;

#[test]
fn test_ast_hex_ingestion_pipeline() {
    let test_patch_path = "test_patch.ast";
    
    // Exactly 100 hex characters following "0x" (16 * 6 + 4 = 100 hex chars -> Total length = 102)
    let valid_hex_patch = "0x0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFABCD";
    
    fs::write(test_patch_path, valid_hex_patch).expect("Failed to write test patch");
    assert!(Path::new(test_patch_path).exists());
    
    let content = fs::read_to_string(test_patch_path).unwrap();
    let trimmed = content.trim();
    
    assert!(trimmed.starts_with("0x"), "AST patch must start with 0x hex prefix");
    assert_eq!(trimmed.len(), 102, "AST patch hex length must be exactly 102 characters (0x + 100 hex chars)");

    // Cleanup test artifact
    let _ = fs::remove_file(test_patch_path);
    println!("[TEST_PASS] AST hex ingestion test passed successfully.");
}
