use sakaya::client::make_xauth;
use std::path::Path;

#[test]
#[ignore]
fn xauth() {
    make_xauth();

    assert!(Path::new("/tmp/.X11-unix/Xauthority").exists());
}
