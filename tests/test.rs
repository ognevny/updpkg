use std::process::Command;

// msys2 ci only

#[test]
fn tarball() {
    Command::new("sh")
        .arg("./.ci/tarball.sh")
        .status()
        .expect("failed to test");
}

// #[test]
// fn git() {

// }
