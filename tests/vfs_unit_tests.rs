use omega_manifold::plan9_vfs::Plan9Vfs;

#[test]
fn test_plan9_vfs_lifecycle() {
    let mut vfs = Plan9Vfs::new("/mnt/omega");
    assert_eq!(vfs.mount_point, "/mnt/omega");
    assert_eq!(vfs.active_channels, 1);

    // Attach a new fid channel
    let res = vfs.attach_fid(42);
    assert!(res.is_ok());
    assert_eq!(vfs.active_channels, 2);
}
