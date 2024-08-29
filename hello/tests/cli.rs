use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8");
    assert_eq!(stdout, "hello world\n");
}
