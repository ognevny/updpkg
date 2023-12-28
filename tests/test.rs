use std::process::Command;

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
