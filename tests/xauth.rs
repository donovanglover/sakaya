use sakaya::client::make_xauth;
use std::path::Path;

#[test]
fn xauth() {
    make_xauth();

    assert!(Path::new("/tmp/container_xauth").exists());
}
