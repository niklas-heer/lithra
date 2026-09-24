use std::process::{Command, Output};

#[allow(
    clippy::expect_used,
    reason = "a test helper; clippy only exempts #[test] functions"
)]
fn lithra(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_lithra"))
        .args(args)
        .output()
        .expect("lithra binary runs")
}

#[test]
fn version_prints_package_version() {
    let output = lithra(&["--version"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("lithra {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn help_lists_usage() {
    let output = lithra(&["--help"]);
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("Usage: lithra")
    );
}

#[test]
fn unknown_argument_fails_with_usage() {
    let output = lithra(&["--nope"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unexpected argument '--nope'"));
    assert!(stderr.contains("Usage: lithra"));
}
